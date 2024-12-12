use crate::common::bson::{
    deserialize_object_id, serialize_datetime, serialize_object_id,
};
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct AccountPermissionDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "account_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub account_id: Option<ObjectId>,
    #[serde(
        rename = "permission_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub permission_id: Option<ObjectId>,
    pub name: String,
    pub value: String,
    #[serde(
        serialize_with = "serialize_datetime"
    )]
    pub created_at: DateTime,
    #[serde(
        serialize_with = "serialize_datetime"
    )]
    pub updated_at: DateTime,
    pub deleted:bool,
}
