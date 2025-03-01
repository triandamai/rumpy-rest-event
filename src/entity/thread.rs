use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::thread_attachment::ThreadAttachment;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Thread {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_by_id: Option<ObjectId>,
    pub quote_thread_id: Option<ObjectId>,
    pub reply_to_thread_id: Option<ObjectId>,
    pub kind:String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub attachment: Vec<ThreadAttachment>,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
