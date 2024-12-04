use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub value: String,
    pub name: String,
    pub group: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
