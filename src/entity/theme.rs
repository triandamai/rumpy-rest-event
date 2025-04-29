use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Theme {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub theme: String,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
