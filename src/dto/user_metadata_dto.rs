use serde::{Deserialize, Serialize};

use crate::entity::user_metadata::UserMetaData;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserMetaDataDTO {
    pub providers: Option<Vec<String>>,
}

impl Into<UserMetaDataDTO> for UserMetaData {
    fn into(self) -> UserMetaDataDTO {
        UserMetaDataDTO {
            providers: self.providers,
        }
    }
}
