use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub branch_name: String,
    pub branch_description: String,
    pub branch_email: Option<String>,
    pub branch_phone_number: Option<String>,
    pub branch_address: Option<String>,
    #[serde(
        rename = "branch_owner",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_owner: Option<ObjectId>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<AccountDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
