use crate::common::bson::*;
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::{thread_attachment_dto::ThreadAttachmentDTO, user_dto::UserDTO};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by")]
    pub created_by: Option<UserDTO>,
    #[serde(
        rename = "quote_thread_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub quote_thread_id: Option<ObjectId>,
    #[serde(rename = "quote_thread")]
    pub quote_thread: Option<ThreadIncludeDTO>,
    #[serde(
        rename = "reply_to_thread_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub reply_to_thread_id: Option<ObjectId>,
    #[serde(rename = "reply_to_thread")]
    pub reply_to_thread: Option<ThreadIncludeDTO>,
    pub kind: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub attachment: Vec<ThreadAttachmentDTO>,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadIncludeDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        // deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        serialize_with = "serialize_object_id",
        // deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(rename = "created_by")]
    pub created_by: Option<UserDTO>,
    #[serde(
        rename = "quote_thread_id",
        serialize_with = "serialize_object_id",
        // deserialize_with = "deserialize_object_id"
    )]
    pub quote_thread_id: Option<ObjectId>,
    #[serde(rename = "quote_thread")]
    pub quote_thread: Option<serde_json::Value>,
    #[serde(
        rename = "reply_to_thread_id",
        serialize_with = "serialize_object_id",
        // deserialize_with = "deserialize_object_id"
    )]
    pub reply_to_thread_id: Option<ObjectId>,
    #[serde(rename = "reply_to_thread")]
    pub reply_to_thread: Option<serde_json::Value>,
    pub kind: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub attachment: Option<Vec<ThreadAttachmentDTO>>,
    pub up_vote_count: Option<i64>,
    pub down_vote_count: Option<i64>,
    pub quote_count: Option<i64>,
    pub reply_count: Option<i64>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub created_at: Option<DateTime>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
}

impl Into<ThreadDTO> for crate::entity::thread::Thread {
    fn into(self) -> ThreadDTO {
        ThreadDTO {
            id: self.id,
            created_by_id: self.created_by_id,
            created_by: None,
            quote_thread_id: self.quote_thread_id,
            quote_thread: None,
            reply_to_thread_id: self.reply_to_thread_id,
            reply_to_thread: None,
            kind: self.kind,
            slug: self.slug,
            title: self.title,
            content: self.content,
            attachment: self
                .attachment
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<ThreadAttachmentDTO>>(),
            up_vote_count: self.up_vote_count,
            down_vote_count: self.down_vote_count,
            quote_count: self.quote_count,
            reply_count: self.reply_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
