use crate::dto::file_attachment_dto::FileAttachmentDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug,Validate)]
pub struct FileAttachment {
    pub id:Option<ObjectId>,
    pub ref_id: Option<ObjectId>,
    pub filename: String,
    pub mime_type: String,
    pub extension: String,
    pub kind: String,
    pub create_at:DateTime,
    pub updated_at:DateTime,
}

impl FileAttachment {
    pub fn to_dto(self)->FileAttachmentDTO{
        FileAttachmentDTO{
            id: self.id,
            ref_id: self.ref_id,
            filename: self.filename,
            mime_type: self.mime_type,
            extension: self.extension,
            kind: self.kind,
            create_at: self.create_at,
            updated_at: self.updated_at,
        }
    }
}