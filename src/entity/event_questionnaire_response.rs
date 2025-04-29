use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventQuestionnaireResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    #[serde(rename = "question_id", skip_serializing_if = "Option::is_none")]
    pub question_id: Option<ObjectId>,
    #[serde(rename = "guest_id", skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<ObjectId>,
    pub answer: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
