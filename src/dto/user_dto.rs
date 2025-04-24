use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::common::bson::*;
use crate::common::serialize::serialize_to_redact_password;
use crate::entity::user::User;

use super::profile_picture_dto::ProfilePictureDTO;
use super::user_metadata_dto::UserMetaDataDTO;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
    #[serde(serialize_with = "serialize_to_redact_password")]
    pub password: Option<String>,
    pub app_meta_data: Option<serde_json::Value>,
    pub user_meta_data: Option<UserMetaDataDTO>,
    pub profile_picture: Option<ProfilePictureDTO>,
    pub last_logged_in: Option<DateTime>,
    pub status: Option<String>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub confirmation_at: Option<DateTime>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub confirmation_sent_at: Option<DateTime>,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            display_name: self.display_name,
            email: self.email,
            phone_number: self.phone_number,
            password: self.password,
            app_meta_data: self.app_meta_data,
            user_meta_data: self.user_meta_data.map_or_else(|| None, |v| Some(v.into())),
            profile_picture: self
                .profile_picture
                .map_or_else(|| None, |value| Some(value.into())),
            last_logged_in: self.last_logged_in,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            confirmation_at: self.confirmation_at,
            confirmation_sent_at: self.confirmation_sent_at,
        }
    }
}
