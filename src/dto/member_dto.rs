use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use chrono::NaiveDate;

#[derive(Serialize,Deserialize, Debug)]
pub struct MemberDTO{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    pub member_code:String,
    #[serde(
        rename = "branch_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id:Option<ObjectId>,
    #[serde(
        rename = "created_by",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by:Option<ObjectId>,
    #[serde(
        rename = "coach_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub coach_id:Option<ObjectId>,
    pub full_name:String,
    pub gender:Option<String>,
    pub email:Option<String>,
    pub date_of_birth:Option<NaiveDate>,
    pub phone_number:Option<String>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}