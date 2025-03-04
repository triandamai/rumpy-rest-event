use crate::entity::user_profile::UserProfile;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct UserProfileDTO {
    pub id:Option<ObjectId>,
    pub bio: Option<String>,
    pub websites: Option<Vec<String>>,
    pub follower:Option<i64>,
    pub following:Option<i64>,
    pub created_at:DateTime,
    pub updated_at:DateTime,
    pub deleted:bool,
}

impl Into<UserProfileDTO> for UserProfile{
    fn into(self) -> UserProfileDTO {
        UserProfileDTO{
            id: self.id,
            bio: self.bio,
            websites: self.websites,
            follower: self.follower,
            following: self.following,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: false,
        }
    }
}