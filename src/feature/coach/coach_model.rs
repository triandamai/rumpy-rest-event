use crate::common::utils::validate_gender;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate,Debug)]
pub struct CreateCoachRequest {
    #[validate(length(min = 0))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 0))]
    pub phone_number: String,
    #[validate(length(min = 0))]
    pub coach_address: String,
    #[validate(custom(function="validate_gender"))]
    pub gender:String,
}

#[derive(Serialize,Deserialize,Validate,Debug)]
pub struct UpdateCoachRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub coach_address: Option<String>,
    #[validate(custom(function="validate_gender"))]
    pub gender:Option<String>,
}
