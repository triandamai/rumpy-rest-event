use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

use super::product_dto::ProductDTO;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailTransactionDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "product_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub product_id: Option<ObjectId>,
    #[serde(rename = "product")]
    pub product: Option<ProductDTO>,
    #[serde(
        rename = "transaction_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub transaction_id: Option<ObjectId>,
    pub kind: String,
    pub notes: String,
    pub quantity: i64,
    pub total_before_discount: f64,
    pub total: f64,
    pub is_membership: bool,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
