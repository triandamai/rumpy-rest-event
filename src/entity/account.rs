use bson::oid::ObjectId;
use bson::DateTime;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Account{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub full_name:String,
    pub email:String,
    pub password:String,
    pub gender:String,
    pub job_title:String,
    pub report_to:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
