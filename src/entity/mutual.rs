use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mutual {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    #[serde(rename = "follower_id", skip_serializing_if = "Option::is_none")]
    pub follower_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
