use bson::oid::ObjectId;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::DateTime;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Member {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub member_id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub created_by:Option<ObjectId>,
    pub coach_id:Option<ObjectId>,
    pub full_name:String,
    pub gender:Option<String>,
    pub email:Option<String>,
    pub date_of_birth:Option<NaiveDate>,
    pub phone_number:Option<String>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at:DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at:DateTime
}