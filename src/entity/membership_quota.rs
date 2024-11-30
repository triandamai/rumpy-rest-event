use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use crate::common::bson::{deserialize_object_id,serialize_object_id};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MembershipQuota{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub membership_id:Option<ObjectId>,
    pub quantity:i64,
    pub created_at:DateTime,
    pub updated_at:DateTime,
}