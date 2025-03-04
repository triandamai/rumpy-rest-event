use crate::dto::user_dto::UserDTO;
use crate::entity::following::Following;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FollowingDTO{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub user: Option<UserDTO>,
    #[serde(rename = "follower_id", skip_serializing_if = "Option::is_none")]
    pub follower_id: Option<ObjectId>,
    pub follower: Option<UserDTO>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<FollowingDTO> for Following {
    fn into(self) -> FollowingDTO {
        FollowingDTO{
            user_id: self.user_id,
            user: None,
            follower_id: self.follower_id,
            follower: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}