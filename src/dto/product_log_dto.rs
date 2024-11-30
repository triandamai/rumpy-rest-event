use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;

#[derive(Serialize,Deserialize, Debug)]
pub struct ProductLogDTO{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    #[serde(
        rename = "branch_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id:Option<ObjectId>,
    pub description:String,
    pub log_type:String,
    pub stock:i64,
    #[serde(
        rename = "created_by",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by:Option<ObjectId>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}