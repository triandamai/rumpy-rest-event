use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoachDTO {
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
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub gender: String,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccountDTO>,
    #[serde(rename = "profile_picture")]
    pub profile_picture: Option<FileAttachmentDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
