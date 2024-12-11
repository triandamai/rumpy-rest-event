use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::utils::{validate_gender,validate_date_of_birth_option};

#[derive(Debug,Clone,Deserialize,Serialize,Validate)]
pub struct CreateMemberRequest{
    #[validate(length(min="1"))]
    pub full_name: String,
    #[validate(custom(function="validate_gender"))]
    pub gender: Option<String>,
    pub email: Option<String>,
    #[validate(custom(function="validate_date_of_birth_option"))]
    pub date_of_birth: Option<String>,
    pub coach_id: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug,Clone,Deserialize,Serialize,Validate)]
pub struct UpdateMemberRequest{
    pub full_name: Option<String>,
    #[validate(custom(function="validate_gender"))]
    pub gender: Option<String>,
    pub email: Option<String>,
    #[validate(custom(function="validate_date_of_birth_option"))]
    pub date_of_birth: Option<String>,
    pub coach_id: Option<String>,
    pub phone_number: Option<String>,
}