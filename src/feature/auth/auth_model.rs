use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::entity::user_credential::{AuthProvider, UserCredential, UserCredentialSecured, UserStatus};

pub const OTP_KEY: &str = "otp";
pub const USER_ID_KEY: &str = "user_id";
pub const USER_UUID_KEY: &str = "user_uuid";
pub const TOKEN_KEY: &str = "token";
pub const ATTEMPT_KEY: &str = "otp_attempt";
pub const RESEND_ATTEMPT_KEY: &str = "resend_attempt";
pub const ISSUED_AT_KEY: &str = "issued_at";
pub const OTP_TTL: i64 = 10800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEmailRequest {
    pub email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInEmailRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub otp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendOtpRequest {
    pub otp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpEmailRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpEmailResponse {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub data: Option<UserCredentialSecured>,
}