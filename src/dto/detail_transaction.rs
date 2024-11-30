use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;

#[derive(Serialize,Deserialize, Debug)]
pub struct DetailTransactionDTO{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    #[serde(
        rename = "product_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub product_id:Option<ObjectId>,
    pub notes:String,
    pub quantity:i64,
    pub total:f64,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}