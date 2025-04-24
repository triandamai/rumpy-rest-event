use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfilePicture {
    pub mime_type: String,
    pub file_name: String,
    pub bucket: String,
}
