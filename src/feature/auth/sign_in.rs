use std::cmp::PartialEq;
use std::sync::mpsc::channel;
use axum::extract::State;
use axum::Json;
use chrono::{ Utc};
use log::info;
use mongodb::bson::{doc, DateTime};
use mongodb::bson::oid::ObjectId;
use redis::RedisResult;
use validator::Validate;
use crate::common::jwt::{JwtClaims, JwtUtil};
use crate::common::{api_response::ApiResponse, app_state::AppState};
use crate::common::smtp::SmtpClient;
use crate::entity::user_credential::UserCredential;
use crate::feature::auth::auth_model::{CheckEmailRequest, SignInEmailRequest, VerifyOtpSignInRequest, ATTEMPT_KEY, ISSUED_AT_KEY, OTP_KEY, RESEND_ATTEMPT_KEY, TOKEN_KEY, USER_ID_KEY};

use super::auth_model::{SignInEmailResponse, VerifyOtpSignInResponse};

pub async fn check_email_exist(
    state: State<AppState>,
    body: Json<CheckEmailRequest>,
) -> ApiResponse<bool> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let exist = UserCredential::email_exist(&body.email, &state.db)
        .await;

    ApiResponse::ok(exist, "Akun bisa digunakan")
}

pub async fn sign_in_email(
    mut state: State<AppState>,
    body: Json<SignInEmailRequest>,
) -> ApiResponse<SignInEmailResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }

    let find_user = UserCredential::find_one(doc! {
        "email":doc!{
            "$eq":&body.email
        }
    }, &state.db).await;

    if find_user.is_none() {
        info!(target: "find user","user not found");
        return ApiResponse::failed("Sign In gagal email atau password tidak sesuai.".to_string());
    }
    let find_user = find_user.unwrap();

    let verify_password = bcrypt::verify(body.password.clone(), find_user.password.clone().as_str());
    if verify_password.is_err() {
        info!(target:"verify password","{}",verify_password.unwrap_err().to_string());
        return ApiResponse::failed("Sign in gagal, email atau password salah.".to_string());
    }
    let verify_password = verify_password.unwrap();
    if !verify_password {
        return ApiResponse::failed("Sign in gagal, email atau password salah.".to_string());
    }
    let mut generate_otp = state.redis.generate_otp();
    if body.is_test_email() {
        generate_otp = "4444".to_string();
    }
    let current_time = Utc::now().timestamp();
    let jwt = JwtUtil::encode(find_user.email.clone());
    if jwt.is_none() {
        return ApiResponse::failed("Gagal membuat sesi.".to_string());
    }
    let jwt = jwt.unwrap();
    let save_otp_session =
        state.redis.set_otp_session_sign_in(
            &find_user.email.as_str(),
            &[
                (ATTEMPT_KEY, "0".to_string()),
                (ISSUED_AT_KEY, format!("{}", current_time)),
                (USER_ID_KEY, find_user.id.unwrap().to_string()),
                (OTP_KEY, generate_otp.clone()),
                (TOKEN_KEY, jwt.clone())
            ],
        );

    if save_otp_session.is_err() {
        return ApiResponse::failed("Gagal membuat sesi.".to_string());
    }

    if !body.is_test_email() {
        let _ = SmtpClient::new(&find_user.email)
            .send(&"[OTP] - Rahasia".to_string(), &generate_otp.clone());
    }

    ApiResponse::ok(
        SignInEmailResponse {
            token: jwt,
        },
        "Otp dikirim ke email kamu",
    )
}

pub async fn verify_otp(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<VerifyOtpSignInRequest>,
) -> ApiResponse<VerifyOtpSignInResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let get_session_from_redis = state.redis.get_session_otp_sign_in(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::un_authorized("Otp tidak valid atau sudah expired");
    }

    let session = get_session_from_redis.unwrap();
    let user_id = session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .to_string();
    let mut attempt = session
        .get(ATTEMPT_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);
    let otp = session
        .get(OTP_KEY)
        .unwrap_or(&String::from("0000"))
        .to_string();

    if attempt > 4 {
        //block user
        return ApiResponse::failed(
            "Kamu sudah mencoba otp terlalu sering, silahkan coba beberapa 3 jam lagi".to_string(),
        );
    }
    let user_id = ObjectId::parse_str(user_id.as_str());
    if user_id.is_err() {
        return ApiResponse::failed("Gagal menemukan sesi.".to_string());
    }
    let user_id = user_id.unwrap();
    let current_time = DateTime::now();
    if !otp.eq(&body.otp.clone()) {
        attempt = attempt + 1;
        let _: RedisResult<String> = state.redis.set_otp_attempt_sign_in(&auth.sub, attempt);


        if attempt > 4 {
            let _ = UserCredential::update_one(
                doc! {"_id":user_id.clone()},
                doc! {"$set":{
                    "status":"Locked",
                    "updated_at":current_time.clone()
                }},
                &state.db,
            ).await;
        }
        return ApiResponse::failed("Verifikasi otp gagal.".to_string());
    }

    let find_user = UserCredential::find_one(
        doc! {
            "_id":user_id.clone()
        },
        &state.db,
    )
        .await;
    if find_user.is_none() {
        return ApiResponse::failed("Tidak dapat menemukan akun.".to_string());
    }
    let find_user = find_user.unwrap();

    let token = JwtUtil::encode(find_user.email.clone());
    if token.is_none() {
        return ApiResponse::failed("gagal membuat sesi.".to_string());
    }
    let token = token.unwrap();
    let current_time = Utc::now().timestamp();
    let save_session = state
        .redis
        .set_session_sign_in(
            &find_user.email.clone(),
            &[
                (ISSUED_AT_KEY, current_time.to_string()),
                (USER_ID_KEY, format!("{}", user_id.clone().to_string())),
                (TOKEN_KEY, token.clone()),
            ],
        );

    ApiResponse::ok(
        VerifyOtpSignInResponse {
            token: token,
            data: None,
        },
        "Login berhasil",
    )
}

pub async fn resend_otp(mut state: State<AppState>, auth: JwtClaims) -> ApiResponse<String> {
    let get_session_from_redis = state.redis.get_session_otp_sign_in(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Sesi tidak ditemukan, silahkan coba login ulang".to_string());
    }

    let session = get_session_from_redis.unwrap();
    let mut resend_otp_attempt = session
        .get(RESEND_ATTEMPT_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i64>()
        .unwrap_or(0);

    if resend_otp_attempt > 4 {
        return ApiResponse::failed(
            "Gagal mengirim ulang otp, kamu sudah mencoba lebih dari 3 kali".to_string(),
        );
    }

    let generate_new_otp = state.redis.generate_otp();

    resend_otp_attempt = resend_otp_attempt + 1;
    let set_otp_to_redis =
        state
            .redis
            .change_otp_session_sign_in(&auth.sub, generate_new_otp, resend_otp_attempt);

    if set_otp_to_redis.is_err() {
        let message = format!("{}", set_otp_to_redis.unwrap_err());
        return ApiResponse::failed(message);
    }

    ApiResponse::ok("".to_string(), "Otp berhasil dikirim ulang")
}
