use bson::DateTime;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use crate::common::bson::{deserialize_object_id,serialize_object_id};
use bson::oid::ObjectId;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Account{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
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
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}
