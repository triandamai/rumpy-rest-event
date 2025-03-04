use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Following{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    #[serde(rename = "follower_id", skip_serializing_if = "Option::is_none")]
    pub follower_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}