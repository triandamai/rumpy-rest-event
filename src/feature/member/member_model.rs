use crate::common::utils::validate_gender;
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateMemberRequest {
    #[validate(length(min = "1"))]
    pub membership_id: String,
    #[validate(length(min = "1"))]
    pub nfc_id: String,
    #[validate(length(min = "1"))]
    pub full_name: String,
    #[validate(custom(function = "validate_gender"))]
    pub gender: Option<String>,
    pub email: Option<String>,
    pub identity_number: Option<String>,
    pub coach_id: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UpdateMemberRequest {
    pub nfc_id: Option<String>,
    pub full_name: Option<String>,
    #[validate(custom(function = "validate_gender"))]
    pub gender: Option<String>,
    pub email: Option<String>,
    pub identity_number: Option<String>,
    pub coach_id: Option<String>,
    pub phone_number: Option<String>,
    pub membership_id: Option<String>,
}

