use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize,Clone, Debug,Validate)]
pub struct FileAttachmentDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    #[serde(
        rename = "ref_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub ref_id: Option<ObjectId>,
    pub filename: String,
    pub mime_type: String,
    pub extension: String,
    pub kind: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_at:DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at:DateTime,
}