use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate,Debug)]
pub struct CreateCoachRequest {
    pub coach_name: String,
    #[validate(email)]
    pub coach_email: String,
    pub coach_phone_number: String,
    pub coach_address: String,
    pub branch_id:String,
    #[validate(length(min = 1,max=1))]
    pub coach_gender:String,
}

#[derive(Serialize,Deserialize,Validate,Debug)]
pub struct UpdateCoachRequest {
    pub coach_id:String,
    pub coach_name: String,
    #[validate(email)]
    pub coach_email: String,
    pub coach_phone_number: String,
    pub coach_address: String,
    pub branch_id:String,
    #[validate(length(min = 1,max=1))]
    pub coach_gender:String,
}
