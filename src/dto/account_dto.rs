use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_permission_dto::AccountPermissionDTO;
use crate::dto::branch_dto::BranchDTO;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub gender: String,
    pub job_title: String,
    #[serde(
        rename = "report_to",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub report_to: Option<ObjectId>,
    #[serde(
        rename = "branch_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id: Option<ObjectId>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountDetailDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub gender: String,
    pub job_title: String,
    #[serde(
        rename = "report_to_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub report_to_id: Option<ObjectId>,
    #[serde(rename = "report_to", skip_serializing_if = "Option::is_none")]
    pub report_to: Option<AccountDTO>,
    #[serde(
        rename = "branch_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id: Option<ObjectId>,
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchDTO>,
    #[serde(rename = "profile_picture", skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<FileAttachmentDTO>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Vec<AccountPermissionDTO>>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
