use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

pub const OTP_KEY: &str = "otp";
pub const BRANCH_ID_KEY: &str = "branch_id";
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

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl SignInRequest {
    pub fn is_test_email(&self) -> bool {
        self.email.eq("triandamai@gmail.com") || self.email.eq("parzival@email.com")
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SignUpEmailResponse {
    pub token: String,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct CompleteSignUpRequest {
    pub full_name: String,
    #[validate(length(min = 10), custom(function = "validate_date_of_birth"))]
    pub date_of_birth: String,
}

impl CompleteSignUpRequest {
    pub fn get_date_of_birth(&self) -> Option<NaiveDate> {
        let parse = NaiveDate::parse_from_str(self.date_of_birth.clone().as_str(), "%Y-%m-%d");
        match parse {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

pub fn validate_date_of_birth(date: &str) -> Result<(), ValidationError> {
    let parse = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    match parse {
        Ok(_) => Ok(()),
        Err(e) => {
            info!(target: "validate_dob::","{:?}",e);
            Err(ValidationError::new(
                "Invalid date of birth format: YYYY-MM-DD",
            ))
        }
    }
}

//change password
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

//forgot password
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

impl ForgotPasswordRequest {
    pub fn is_test_email(&self) -> bool {
        self.email.eq("trian1@email.com") || self.email.eq("parzival@email.com")
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CompleteForgotPasswordRequest {
    #[validate(length(min=1))]
    pub new_password: String,
}

