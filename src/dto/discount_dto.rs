use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub title: String,
    pub amount: f64,
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
