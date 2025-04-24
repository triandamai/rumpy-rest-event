use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfilePictureDTO {
    pub mime_type: String,
    pub path: String,
    pub file_name: String,
    pub bucket: String,
}
