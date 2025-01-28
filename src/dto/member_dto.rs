use crate::common::bson::{
    deserialize_object_id, serialize_datetime, serialize_file_attachment, serialize_object_id,
};
use crate::dto::account_dto::AccountDTO;
use crate::dto::branch_dto::BranchDTO;
use crate::dto::coach_dto::CoachDTO;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::dto::member_subscription_dto::MemberSubscriptionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub member_code: String,
    #[serde(
        rename = "branch_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id: Option<ObjectId>,
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchDTO>,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccountDTO>,
    #[serde(
        rename = "coach_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub coach_id: Option<ObjectId>,
    #[serde(rename = "subscription")]
    pub subscription: Option<MemberSubscriptionDTO>,
    #[serde(rename = "coach", skip_serializing_if = "Option::is_none")]
    pub coach: Option<CoachDTO>,
    pub nfc_id: Option<String>,
    pub full_name: String,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub identity_number: Option<String>,
    pub phone_number: Option<String>,
    #[serde(
        rename = "profile_picture",
        serialize_with = "serialize_file_attachment"
    )]
    pub profile_picture: Option<FileAttachmentDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
