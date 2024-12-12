use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use crate::dto::branch_dto::BranchDTO;
use crate::dto::coach::CoachDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::dto::file_attachment_dto::FileAttachmentDTO;

#[derive(Serialize,Deserialize, Debug)]
pub struct MemberDTO{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id:Option<ObjectId>,
    pub member_code:String,
    #[serde(
        rename = "branch_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub branch_id:Option<ObjectId>,
    pub branch:Option<BranchDTO>,
    #[serde(
        rename = "created_by",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id:Option<ObjectId>,
    pub created_by:Option<AccountDTO>,
    #[serde(
        rename = "coach_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub coach_id:Option<ObjectId>,
    pub coach:Option<CoachDTO>,
    pub full_name:String,
    pub gender:Option<String>,
    pub email:Option<String>,
    pub identity_number:Option<String>,
    pub phone_number:Option<String>,
    pub profile_picture:Option<FileAttachmentDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted:bool,
}

