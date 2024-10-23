use chrono::{DateTime, NaiveDate, ParseResult, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::entity::user_credential::{UserCredential};

pub const OTP_KEY: &str = "otp";
pub const USER_ID_KEY: &str = "user_id";
pub const USER_UUID_KEY: &str = "user_uuid";
pub const TOKEN_KEY: &str = "token";
pub const ATTEMPT_KEY: &str = "otp_attempt";
pub const RESEND_ATTEMPT_KEY: &str = "resend_attempt";
pub const ISSUED_AT_KEY: &str = "issued_at";
pub const OTP_TTL: i64 = 10800;
pub const SIGN_IN_TTL: i64 = 10800;
pub const SIGN_UP_TTL: i64 = 10800;

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CheckEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Validate,Serialize, Deserialize)]
pub struct SignInEmailRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl SignInEmailRequest {
    pub fn is_test_email(&self) -> bool {
        self.email.eq("triandamai@gmail.com") || self.email.eq("parzival@email.com")
    }
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct SignInEmailResponse {
    pub token: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpSignInRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpSignInResponse {
    pub token: String,
    pub data: Option<UserCredential>,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct ResendOtpSignInRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}

//sign up
#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct SignUpEmailRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}


impl SignUpEmailRequest {
    pub fn is_test_email(&self) -> bool {
        self.email.eq("triandamai@gmail.com") || self.email.eq("parzival@email.com")
    }
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct SignUpEmailResponse {
    pub token: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpSignUpRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpSignUpResponse {
    pub token: String,
    pub data: Option<UserCredential>,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct ResendOtpSignUpRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}

#[derive(Debug,Validate, Clone, Serialize, Deserialize)]
pub struct CompleteSignUpRequest {
    pub full_name: String,
    pub username: String,
    #[validate(length(min = 10), custom(function = "validate_date_of_birth"))]
    pub date_of_birth: String,
}

impl CompleteSignUpRequest {
    pub fn get_date_of_birth(&self) -> Option<NaiveDate> {
        let parse = NaiveDate::parse_from_str(self.date_of_birth.clone().as_str(), "%Y-%m-%d");
        match parse {
            Ok(value) => {
                Some(value)
            }
            Err(_) => {
                None
            }
        }
    }
}

pub fn validate_date_of_birth(date: &str) -> Result<(), ValidationError> {
    let parse = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    match parse {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            info!(target: "validate_dob::","{:?}",e);
            Err(ValidationError::new("Invalid date of birth format: YYYY-MM-DD"))
        }
    }
}
#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct CompleteSignUpResponse {
    pub token: String,
    pub data: Option<UserCredential>,
}

//forgot password
#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

impl ForgotPasswordRequest {
    pub fn is_test_email(&self) -> bool {
        self.email.eq("trian1@email.com") || self.email.eq("parzival@email.com")
    }
}


#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct ForgotPasswordResponse {
    pub token: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpForgotPasswordRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}
#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct VerifyOtpForgotPasswordResponse {
    pub token: String,
    pub data: Option<UserCredential>,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct ResendOtpForgotPasswordRequest {
    #[validate(length(min = 4, max = 4))]
    pub otp: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct CompleteForgotPasswordRequest {
    pub new_password: String,
}

#[derive(Debug, Clone,Validate, Serialize, Deserialize)]
pub struct CompleteForgotPasswordResponse {
    pub token: String,
}
