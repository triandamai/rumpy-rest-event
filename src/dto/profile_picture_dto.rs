use crate::entity::profile_picture::ProfilePicture;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfilePictureDTO {
    pub mime_type: String,
    pub file_name: String,
    pub bucket: String,
}

impl Into<ProfilePictureDTO> for ProfilePicture {
    fn into(self) -> ProfilePictureDTO {
        ProfilePictureDTO {
            mime_type: self.mime_type,
            file_name: self.file_name,
            bucket: self.bucket,
        }
    }
}
