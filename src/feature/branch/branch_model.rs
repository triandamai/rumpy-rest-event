use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Validate,Serialize,Deserialize)]
pub struct CreateBranchRequest{
    #[validate(length(min = 1))]
    pub branch_name:String,
    #[validate(length(min = 1))]
    pub branch_description:String,
    pub branch_email:Option<String>,
    pub branch_phone_number:Option<String>,
    pub branch_address:Option<String>
}

#[derive(Debug,Validate,Serialize,Deserialize)]
pub struct UpdateBranchRequest{
    #[validate(length(min = 1))]
    pub branch_name:String,
    #[validate(length(min = 1))]
    pub branch_description:String,
    pub branch_email:Option<String>,
    pub branch_phone_number:Option<String>,
    pub branch_address:Option<String>
}