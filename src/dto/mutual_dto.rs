use crate::dto::user_dto::UserDTO;
use crate::entity::mutual::Mutual;
use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MutualDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub user: Option<UserDTO>,
    #[serde(rename = "mutual_id", skip_serializing_if = "Option::is_none")]
    pub mutual_id: Option<ObjectId>,
    pub mutual: Option<UserDTO>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<MutualDTO> for Mutual {
    fn into(self) -> MutualDTO {
        MutualDTO {
            id: self.id,
            user_id: self.user_id,
            user: None,
            mutual_id: self.mutual_id,
            mutual: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
