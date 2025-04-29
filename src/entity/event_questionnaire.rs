use std::collections::HashMap;

use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventQuestionnaire {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    pub name: String,
    pub question: String,
    pub ordering: i64,
    pub question_type: String,
    pub options: HashMap<String, String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
