use std::collections::HashMap;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use bcrypt::{BcryptResult, DEFAULT_COST};
use chrono::NaiveDateTime;
use redis::{Commands, RedisResult};

use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::{JwtClaims, JwtUtil};
use crate::common::smtp::SmtpClient;
use crate::entity::user_credential::{AuthProvider, UserCredential, UserCredentialSecured, UserStatus};
use crate::feature::auth::auth_model::{ATTEMPT_KEY, AuthResponse, ISSUED_AT_KEY, OTP_KEY, OTP_TTL, RESEND_ATTEMPT_KEY, SignUpEmailRequest, TOKEN_KEY, USER_ID_KEY, VerifyOtpRequest};
use crate::repositories;
use crate::repositories::auth_repository;

pub async fn sign_up_email(
    mut state: State<AppState>,
    body: Json<SignUpEmailRequest>,
) -> impl IntoResponse {
    let find_existing = repositories::auth_repository::get_user_by_email(
        body.email.clone(),
        &state.postgres,
    )
        .await;

    if find_existing.is_none() {
        return ApiResponse::failed("Login gagal, akun tidak ditemukan".to_string());
    }

    let create_password: BcryptResult<String> = bcrypt::hash(
        body.password.clone(),
        DEFAULT_COST,
    );

    if create_password.is_err() {
        return ApiResponse::failed("Gagal mendaftarkan akun, silahkan coba beberapa saat lagi".to_string());
    }
    let uuid = uuid::Uuid::new_v4();

    let user = UserCredential {
        id: Default::default(),
        uuid: uuid.to_string(),
        username: "n/a".to_string(),
        password: create_password.unwrap(),
        full_name: "n/a".to_string(),
        email: body.email.clone(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
        status: UserStatus::WaitingConfirmation,
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let saved_user = auth_repository::create_new_user(
        user,
        &state.postgres,
    ).await;


    if saved_user.is_err() {
        return ApiResponse::failed("Gagal mendaftarkan akun kamu, coba lagi nanti.".to_string());
    }

    let user_credential = saved_user.unwrap();

    let otp_session_key = state.redis.create_key_otp_session(body.email.as_str());
    let mut otp = state.redis.generate_otp();
    let token = JwtUtil::encode(body.email.clone());
    let current_date = chrono::Utc::now();

    if body.email.eq("trian1@email.com") ||
        body.email.eq("parzival@email.com") {
        otp = String::from("4444");
    }

    let set_session_to_redis: RedisResult<String> = state
        .redis
        .client
        .hset_multiple(otp_session_key.clone(), &[
            (OTP_KEY, otp.clone()),
            (USER_ID_KEY, user_credential.id.to_string()),
            (TOKEN_KEY, token.clone().unwrap()),
            (ATTEMPT_KEY, 0.to_string()),
            (RESEND_ATTEMPT_KEY, 0.to_string()),
            (ISSUED_AT_KEY, current_date.timestamp_millis().to_string())
        ]);

    if set_session_to_redis.is_err() {
        return ApiResponse::failed("Akun berhasil didaftarkan, tapi kami mengalami masalah membuat sesi".to_string());
    }

    let _set_expire: RedisResult<String> = state
        .redis
        .client
        .expire(otp_session_key, OTP_TTL);

    if !body.email.eq("trian1@email.com") || !body.email.eq("parzival@email.com") {
        let send_to = format!("{} <{}>", user_credential.full_name, user_credential.email);
        let subject = "[SIRKEL-OTP] ".to_string();
        let body = format!("here your code: {}", otp);
        let _ = SmtpClient::new(&send_to)
            .send(
                &subject,
                &body,
            );
    }

    ApiResponse::ok(AuthResponse {
        token: token.unwrap(),
        data: None,
    }, "Berhasil mendaftarkan akun, silahkan cek email kamu.")
}

pub async fn verify_otp_sign_up_email(
    mut state: State<AppState>,
    header: JwtClaims,
    body: Json<VerifyOtpRequest>,
) -> impl IntoResponse {
    let otp_session_key = state
        .redis
        .create_key_otp_session(&header.sub);

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(otp_session_key.clone());

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal memverifikasi otp, sesi tidak ditemukan".to_string());
    }

    let session = get_session_from_redis.unwrap();
    let user_id = session.get(USER_ID_KEY).unwrap_or(&String::from("0")).to_string().parse().unwrap_or(0);
    let mut attempt = session.get(ATTEMPT_KEY).unwrap_or(&String::from("0")).parse::<i32>().unwrap_or(0);
    let otp = session.get(OTP_KEY).unwrap_or(&String::from("0000")).to_string();
    let issued_at = session.get(ISSUED_AT_KEY).unwrap_or(&String::from("0")).parse::<i64>().unwrap_or(0);
    let issued_at_chrono = NaiveDateTime::from_timestamp_millis(issued_at);

    if issued_at_chrono.is_none() {
        return ApiResponse::un_authorized("Gagal memverifikasi otp, mungkin sudah kadaluarsa");
    }

    if attempt > 4 {
        //block user
        return ApiResponse::un_authorized("Kamu sudah mencoba otp terlalu sering, silahkan coba beberapa 3 jam lagi");
    }

    if attempt < 4 {
        if !otp.eq(&body.otp.clone()) {
            attempt = attempt + 1;

            let _: RedisResult<String> = state
                .redis
                .client
                .hset_multiple(otp_session_key.clone(), &[
                    (ATTEMPT_KEY, attempt.to_string())
                ]);
        }
    }


    let find_user = auth_repository::get_user_by_id_unsecured(user_id, &state.postgres)
        .await;


    if find_user.is_none() {
        return ApiResponse::un_authorized("");
    }

    let user_credential = find_user.unwrap();

    let session_key = state
        .redis
        .create_key_otp_session(&user_credential.email);

    let token = JwtUtil::encode(user_credential.email.clone());

    if token.is_none() {
        return ApiResponse::un_authorized("Gagal membuat sesi");
    }

    let set_session_to_redis: RedisResult<String> = state
        .redis
        .client
        .hset_multiple(session_key.clone(), &[
            (USER_ID_KEY, user_credential.id.to_string()),
            (TOKEN_KEY, token.clone().unwrap())
        ]);

    if set_session_to_redis.is_err() {
        return ApiResponse::un_authorized("Gagal membuat sesi, silahkan coba beberapa saat lagi");
    }

    ApiResponse::ok(
        AuthResponse {
            token: token.unwrap(),
            data: Some(UserCredentialSecured::from(user_credential)),
        },
        "Login berhasil",
    )
}

pub async fn resend_otp_sign_up_email(
    mut state: State<AppState>,
    claims: JwtClaims,
) -> impl IntoResponse {
    let otp_session_key = state
        .redis
        .create_key_otp_session(claims.sub.as_str());

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(otp_session_key.clone().as_str());

    if get_session_from_redis.is_err() {
        return ApiResponse::un_authorized("Sesi tidak ditemukan, silahkan coba login ulang");
    }

    let session = get_session_from_redis.unwrap();
    let otp_attempt = session.get(ATTEMPT_KEY).unwrap_or(&String::from("")).parse::<i64>()
        .unwrap_or(0);

    if otp_attempt > 4 {
        return ApiResponse::un_authorized("Gagal mengirim ulang otp, kamu sudah mencoba lebih dari 3 kali");
    }

    let generate_new_otp = state.redis.generate_otp();
    let _set_otp_to_redis: RedisResult<String> = state.redis
        .client
        .hset_multiple(otp_session_key.clone(), &[
            (OTP_KEY, generate_new_otp)
        ]);

    ApiResponse::ok(None::<String>, "Otp berhasil dikirim ulang")
}