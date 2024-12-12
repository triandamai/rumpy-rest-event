use crate::common::utils::validate_gender;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 1), custom(function = "validate_gender"))]
    pub gender: String,
    #[validate(length(min = 7))]
    pub password: String,
    pub job_title: String,
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    #[validate(custom(function = "validate_gender"))]
    pub gender: Option<String>,
    pub job_title: Option<String>,
}
