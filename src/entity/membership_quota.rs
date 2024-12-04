use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

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