use axum::extract::State;
use axum::Json;
use bcrypt::DEFAULT_COST;
use chrono::{Utc};
use log::info;
use mongodb::bson::{doc, DateTime};
use mongodb::bson::oid::ObjectId;
use validator::Validate;
use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::{JwtClaims, JwtUtil};
use crate::common::smtp::SmtpClient;
use crate::entity::user_credential::{AuthProvider, UserCredential, UserStatus};
use crate::feature::auth::auth_model::{SignUpEmailRequest, ATTEMPT_KEY, ISSUED_AT_KEY, OTP_KEY, RESEND_ATTEMPT_KEY, TOKEN_KEY, USER_ID_KEY};


use super::auth_model::{
    CompleteSignUpRequest, CompleteSignUpResponse, SignUpEmailResponse, VerifyOtpSignUpRequest,
    VerifyOtpSignUpResponse,
};

pub async fn sign_up_email(
    mut state: State<AppState>,
    body: Json<SignUpEmailRequest>,
) -> ApiResponse<SignUpEmailResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let is_exist = UserCredential::email_exist(&body.email, &state.db)
        .await;
    if !is_exist {
        info!(target: "sign_up_email::","email {} already exist",body.email.clone());
        return ApiResponse::failed("Maaf pendaftaran gagal, email sudah terdafatar.".to_string());
    }

    let create_password = bcrypt::hash(body.password.clone(), DEFAULT_COST);
    if create_password.is_err() {
        info!(target: "sign_up_email::","failed create password -> {}",create_password.unwrap_err().to_string());
        return ApiResponse::failed("Gagal membuat password".to_string());
    }
    let create_password = create_password.unwrap();
    let current_time = DateTime::now();
    let mut create_user = UserCredential {
        id: None,
        full_name: "".to_string(),
        email: body.email.clone(),
        password: create_password,
        status: UserStatus::WaitingConfirmation,
        date_of_birth: None,
        created_at: current_time.clone(),
        updated_at: current_time.clone(),
        username: "".to_string(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
    };

    let create_user = create_user.save(&state.db).await;
    if create_user.is_err() {
        return ApiResponse::failed(create_user.unwrap_err());
    }
    let create_user = create_user.unwrap();

    let mut generate_otp = state.redis.generate_otp();
    if body.is_test_email() {
        generate_otp = String::from("4444");
    }
    let current_time = Utc::now().timestamp();

    let jwt = JwtUtil::encode(body.email.clone());
    if jwt.is_none() {
        return ApiResponse::failed("Gagal membuat sesi.".to_string());
    }
    let jwt = jwt.unwrap();

    let save_session = state
        .redis
        .set_otp_session_sign_up(
            body.email.as_str(),
            &[
                (ATTEMPT_KEY, "0".to_string()),
                (ISSUED_AT_KEY, format!("{}", current_time)),
                (USER_ID_KEY, create_user.id.unwrap().to_string()),
                (OTP_KEY, generate_otp.clone()),
                (TOKEN_KEY, jwt.clone())
            ],
        );

    if save_session.is_err() {
        info!(target: "sign_up_email::save_session","{}",save_session.unwrap_err().to_string());
        return ApiResponse::failed("Gagal membuat sesi.".to_string());
    }


    if !body.is_test_email() {
        let _ = SmtpClient::new(&body.email.clone())
            .send(&"[OTP] - Rahasia".to_string(), &generate_otp)
            .await;
    }

    ApiResponse::ok(
        SignUpEmailResponse {
            token: jwt
        },
        "Berhasil mendaftarkan akun, silahkan cek email kamu.",
    )
}

pub async fn verify_otp(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<VerifyOtpSignUpRequest>,
) -> ApiResponse<VerifyOtpSignUpResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let get_session_from_redis = state.redis.get_session_otp_sign_up(&auth.sub);

    if get_session_from_redis.is_err() {
        info!(target: "verify_otp::get_session","{}",get_session_from_redis.unwrap_err().to_string());
        return ApiResponse::failed("Gagal memverifikasi otp, sesi tidak ditemukan".to_string());
    }

    let session = get_session_from_redis.unwrap();
    // info!(target: "verify_otp::get_session","{:?}",session);
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
    let issued_at = session
        .get(ISSUED_AT_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i64>()
        .unwrap_or(0);

    if attempt > 4 {
        //block user
        info!(target: "sign_up_email::save_session","to much trying attempt.");
        return ApiResponse::failed(
            "Kamu sudah mencoba otp terlalu sering, silahkan coba beberapa 3 jam lagi".to_string(),
        );
    }

    if !otp.eq(&body.otp.clone()) {
        attempt = attempt + 1;
        let _ = state.redis.set_otp_attempt_sign_up(&auth.sub, attempt);
        if attempt > 4 {
            let _ = UserCredential::update_one(
                doc! {
                    "_id":doc!{
                        "$eq":user_id.clone(),
                    }
                }, doc! {"status":"Locked"},
                &state.db).await;
        }
        info!(target: "sign_up_email::save_session","otp not same {} == {}",otp.clone(),body.otp.clone());
        return ApiResponse::failed("Verifikasi otp gagal.".to_string());
    }

    let id = ObjectId::parse_str(user_id.as_str()).unwrap();
    let find_user = UserCredential::find_one(
        doc! {"_id":id.clone()},
        &state.db,
    ).await;


    if find_user.is_none() {
        return ApiResponse::failed("Tidak dapat menemukan user".to_string());
    }
    let find_user = find_user.unwrap();
    if !find_user.is_waiting_confirmation() {
        return ApiResponse::failed(find_user.get_status_message().to_string());
    }

    let update_user = UserCredential::update_one(
        doc! { "_id":id.clone()},
        doc! {"$set":doc!{"status":"Active"}},
        &state.db,
    ).await;

    if update_user.is_err() {
        return ApiResponse::failed(update_user.unwrap_err());
    }

    let jwt = JwtUtil::encode(find_user.email.clone());


    let jwt = jwt.unwrap();
    let save_session = state
        .redis
        .set_session_sign_in(
            find_user.email.clone().as_str(), &[
                (ISSUED_AT_KEY, format!("{}", issued_at)),
                (USER_ID_KEY, format!("{}", user_id)),
                (TOKEN_KEY, jwt.clone()),
            ],
        );
    if save_session.is_err() {
        return ApiResponse::failed("Gagal membuat sesi.".to_string());
    }
    let _ = state.redis.delete_otp_session_sign_up(&auth.sub);

    ApiResponse::ok(
        VerifyOtpSignUpResponse {
            token: jwt.clone(),
            data: None,
        },
        "Login berhasil",
    )
}

pub async fn resend_otp(mut state: State<AppState>, auth: JwtClaims) -> ApiResponse<String> {
    let get_session_from_redis = state.redis.get_session_otp_sign_up(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Sesi tidak ditemukan, silahkan coba login ulang".to_string());
    }

    let session = get_session_from_redis.unwrap();
    let mut resend_otp_attempt = session
        .get(RESEND_ATTEMPT_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
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
            .change_otp_session_sign_up(&auth.sub, generate_new_otp, resend_otp_attempt);


    if set_otp_to_redis.is_err() {
        let message = format!("{}", set_otp_to_redis.unwrap_err());

        return ApiResponse::failed(message);
    }


    ApiResponse::ok("".to_string(), "Otp berhasil dikirim ulang")
}

pub async fn complete_sign_up(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CompleteSignUpRequest>,
) -> ApiResponse<CompleteSignUpResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let get_session_from_redis = state.redis.get_session_sign_in(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::un_authorized("Sesi tidak ditemukan");
    }
    let session = get_session_from_redis.unwrap();
    let user_id = session
        .get(USER_ID_KEY);
    if user_id.is_none() {
        return ApiResponse::failed("Tidak menemukan sesi.".to_string());
    }
    let user_id = user_id.unwrap();
    let user_id = ObjectId::parse_str(user_id.as_str()).unwrap();

    let find_user = UserCredential::find_one(
        doc! {"_id":user_id.clone()},
        &state.db,
    ).await;

    if find_user.is_none() {
        return ApiResponse::failed("User tidak ditemukan.".to_string());
    }

    let current_time = DateTime::now().try_to_rfc3339_string();
    if current_time.is_err() {
        return ApiResponse::failed(current_time.unwrap_err().to_string());
    }
    let update_user = UserCredential::update_one(
        doc! {"_id":user_id.clone()},
        doc! {
            "$set":doc! {
                "status":"Active",
                "username": body.username.clone(),
                "full_name": body.full_name.clone(),
                "date_of_birth":body.date_of_birth.clone().to_string(),
                "updated_at":current_time.unwrap(),
            }
        },
        &state.db,
    ).await;

    if update_user.is_err() {
        return ApiResponse::failed(update_user.unwrap_err());
    }

    ApiResponse::ok(
        CompleteSignUpResponse {
            token: "".to_string(),
            data: None,
        },
        "Sign Up success",
    )
}
