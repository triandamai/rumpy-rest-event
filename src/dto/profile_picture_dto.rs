use crate::{common::bson::*, entity::profile_picture::ProfilePicture};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfilePictureDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub mime_type: String,
    pub file_name: String,
    pub bucket_name: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

impl Into<ProfilePictureDTO> for ProfilePicture {
    fn into(self) -> ProfilePictureDTO {
        ProfilePictureDTO {
            id: self.id,
            mime_type: self.mime_type,
            file_name: self.file_name,
            bucket_name: self.bucket_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
