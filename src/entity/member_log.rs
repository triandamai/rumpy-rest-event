use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use crate::common::bson::{deserialize_object_id,serialize_object_id};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MemberLog{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    pub member_id:Option<ObjectId>,
    pub name:String,
    pub value:String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at:DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at:DateTime,
}