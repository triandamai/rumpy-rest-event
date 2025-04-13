use std::collections::HashMap;

use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(rename = "ref_id", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<ObjectId>,
    pub kind: String,
    pub title: String,
    pub body: Option<NotificationContent>,
    pub notification_type: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NotificationContent {
    pub content: String,
    pub extend_value: serde_json::Value,
    pub formatter_value: HashMap<String, String>,
}
