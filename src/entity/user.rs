use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::common::serialize::serialize_to_redact_password;

use super::{profile_picture::ProfilePicture, user_metadata::UserMetaData};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub app_meta_data: Option<serde_json::Value>,
    pub user_meta_data: Option<UserMetaData>,
    pub profile_picture: Option<ProfilePicture>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub confirmation_at: Option<DateTime>,
    pub confirmation_sent_at: Option<DateTime>,
}
