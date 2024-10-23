use super::auth_model::{
    CompleteForgotPasswordRequest, CompleteForgotPasswordResponse, ForgotPasswordRequest,
    ForgotPasswordResponse, VerifyOtpForgotPasswordRequest, VerifyOtpForgotPasswordResponse,
};
use crate::common::jwt::{JwtClaims, JwtUtil};
use crate::common::smtp::SmtpClient;
use crate::common::{api_response::ApiResponse, app_state::AppState};
use crate::entity::user_credential::{UserStatus};
use crate::feature::auth::auth_model::{
    ATTEMPT_KEY, ISSUED_AT_KEY, OTP_KEY, RESEND_ATTEMPT_KEY, TOKEN_KEY, USER_ID_KEY,
};
use axum::{extract::State, Json};
use bcrypt::{BcryptResult, DEFAULT_COST};
use chrono::DateTime;
use log::info;

pub async fn forgot_password(
    mut state: State<AppState>,
    body: Json<ForgotPasswordRequest>,
) -> ApiResponse<ForgotPasswordResponse> {
    info!(target:"sign_in_email","{:?}",body.clone());

    ApiResponse::ok(
        ForgotPasswordResponse {
            token: "".to_string(),
        },
        "Otp dikirim ke email kamu",
    )
}

pub async fn verify_otp(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<VerifyOtpForgotPasswordRequest>,
) -> ApiResponse<VerifyOtpForgotPasswordResponse> {
    let get_session_from_redis = state.redis.get_session_otp_forgot_password(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal memverifikasi otp, sesi tidak ditemukan".to_string());
    }

    let session = get_session_from_redis.unwrap();
    let user_id = session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .to_string()
        .parse()
        .unwrap_or(0);
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
    let issued_at_chrono = DateTime::from_timestamp_millis(issued_at);

    if issued_at_chrono.is_none() {
        return ApiResponse::un_authorized(
            "Gagal memverifikasi otp, mungkin sudah kadaluarsa"
        );
    }

    if attempt > 4 {
        //block user
        return ApiResponse::un_authorized(
            "Kamu sudah mencoba otp terlalu sering, silahkan coba beberapa 3 jam lagi",
        );
    }

    if !otp.eq(&body.otp.clone()) {
        attempt = attempt + 1;

        let _ = state
            .redis
            .set_otp_attempt_forgot_password(&auth.sub, attempt);
        return ApiResponse::failed("Verifikasi otp gagal.".to_string());
    }


    ApiResponse::ok(
        VerifyOtpForgotPasswordResponse {
            token: "".to_string(),
            data:None,
        },
        "Reset password berhasil",
    )
}

pub async fn resend_otp(mut state: State<AppState>, auth: JwtClaims) -> ApiResponse<String> {
    let get_session_from_redis = state
        .redis
        .get_session_otp_forgot_password(&auth.sub);

    if get_session_from_redis.is_err() {
        return ApiResponse::un_authorized("Sesi tidak ditemukan, silahkan coba login ulang");
    }

    let session = get_session_from_redis.unwrap();
    let mut resend_otp_attempt = session
        .get(RESEND_ATTEMPT_KEY)
        .unwrap_or(&String::from(""))
        .parse::<i64>()
        .unwrap_or(0);

    if resend_otp_attempt > 4 {
        return ApiResponse::un_authorized(
            "Gagal mengirim ulang otp, kamu sudah mencoba lebih dari 3 kali",
        );
    }

    let generate_new_otp = state.redis.generate_otp();
    resend_otp_attempt = resend_otp_attempt + 1;
    let set_otp_to_redis = state.redis.change_otp_session_forgot_password(
        &auth.sub,
        generate_new_otp,
        resend_otp_attempt,
    );

    if set_otp_to_redis.is_err() {
        let message = format!("{}", set_otp_to_redis.unwrap_err());

        return ApiResponse::failed(message);
    }

    ApiResponse::ok("".to_string(), "Otp berhasil dikirim ulang")
}

pub async fn complete_forgot_password(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CompleteForgotPasswordRequest>,
) -> ApiResponse<CompleteForgotPasswordResponse> {
    let get_session_from_redis = state.redis.get_session_sign_up(
        &auth.sub.clone()
    );

    if get_session_from_redis.is_err() {
        return ApiResponse::un_authorized("Sesi tidak ditemukan");
    }
    let session = get_session_from_redis.unwrap();
    let user_id = session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("-1"))
        .parse::<i32>()
        .unwrap_or(-1);


    let create_password: BcryptResult<String> =
        bcrypt::hash(body.new_password.clone(), DEFAULT_COST);

    if create_password.is_err() {
        return ApiResponse::failed(
            "Gagal membuat password baru, coba bebrapa saat lagi.".to_string(),
        );
    }



    ApiResponse::ok(
        CompleteForgotPasswordResponse {
            token:"".to_string(),
        },
        "Password has been reset,try login with new password.",
    )
}
