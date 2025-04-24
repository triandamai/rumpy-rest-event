use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NotificationLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(rename = "notification_id", skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<ObjectId>,
    #[serde(rename = "ref_id", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<ObjectId>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub is_read: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
