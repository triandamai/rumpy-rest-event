use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use crate::dto::branch_dto::BranchDTO;
use crate::dto::product_dto::ProductDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductLogDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "branch_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id: Option<ObjectId>,
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchDTO>,
    #[serde(
        rename = "product_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub product_id: Option<ObjectId>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductDTO>,
    pub description: String,
    pub log_type: String,
    pub stock: i64,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccountDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
