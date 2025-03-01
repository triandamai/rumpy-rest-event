use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::entity::thread_vote::ThreadVote;

use super::{thread_dto::ThreadDTO, user_dto::UserDTO};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadVoteDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by")]
    pub created_by: Option<UserDTO>,
    pub thread_id: Option<ObjectId>,
    #[serde(rename = "thread")]
    pub thread: Option<ThreadDTO>,
    pub kind: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<ThreadVoteDTO> for ThreadVote {
    fn into(self) -> ThreadVoteDTO {
        ThreadVoteDTO {
            id: self.id,
            created_by_id: self.created_by_id,
            thread_id: self.thread_id,
            kind: self.kind,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by: None,
            thread: None,
        }
    }
}
