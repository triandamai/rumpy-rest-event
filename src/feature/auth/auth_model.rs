use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::user_dto::UserDTO;

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct AuthOTPRequest {
    #[validate(length(min = 4))]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct VerifyOTPRequest {
    #[validate(length(min = 4))]
    pub otp: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct SignInEmailRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ResetPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthOTPResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyResetPasswordResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNewPasswordRequest {
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyOTPResponse {
    pub token: String,
    pub refresh_token: String,
    pub storage_token: String,
    pub account: UserDTO,
    pub auth_type:String
}
