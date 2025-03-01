use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadVote {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_by_id: Option<ObjectId>,
    pub thread_id: Option<ObjectId>,
    pub kind: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
