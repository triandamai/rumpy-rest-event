use crate::{common::bson::*, entity::thread_attachment::ThreadAttachment};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThreadAttachmentDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub kind: String,
    pub mime_type: String,
    pub bucket_name: String,
    pub file_name: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

impl Into<ThreadAttachmentDTO> for ThreadAttachment {
    fn into(self) -> ThreadAttachmentDTO {
        ThreadAttachmentDTO {
            id: self.id,
            kind: self.kind,
            mime_type: self.mime_type,
            bucket_name: self.bucket_name,
            file_name: self.file_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<ThreadAttachment> for ThreadAttachmentDTO {
    fn into(self) -> ThreadAttachment {
        ThreadAttachment {
            id: self.id,
            kind: self.kind,
            mime_type: self.mime_type,
            bucket_name: self.bucket_name,
            file_name: self.file_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
