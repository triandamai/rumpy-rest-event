use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::account_dto::AccountDTO;
use crate::dto::member_dto::MemberDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberLogDTO {
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
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<MemberDTO>,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccountDTO>,
    pub name: String,
    pub value: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    pub deleted: bool,
}
