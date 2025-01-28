use crate::common::bson::{deserialize_object_id, serialize_datetime, serialize_object_id};
use crate::dto::membership_dto::MembershipDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberSubscriptionDTO {
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
    #[serde(
        rename = "membership_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub membership_id: Option<ObjectId>,
    #[serde(rename = "membership")]
    pub membership: Option<MembershipDTO>,
    pub balance: f64,
    pub outstanding_balance: f64,
    pub quota: i64,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub update_at: DateTime,
}
