use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::member_dto::MemberDTO;
use crate::dto::product_dto::ProductDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberCartDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "member_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub member_id: Option<ObjectId>,
    pub member: Option<MemberDTO>,
    pub notes: String,
    #[serde(
        rename = "product_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub product_id: Option<ObjectId>,
    pub product: Option<ProductDTO>,
    pub quantity: i64,
    pub discount: f64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}
