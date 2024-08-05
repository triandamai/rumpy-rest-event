
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use crate::entity::file::Storage;
use crate::entity::user_credential::{UserCredential, UserCredentialSecured};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Space {
    pub id: i32,
    pub user_id: i32, //created_by
    pub name: String,
    pub space_thumbnail_id: Option<i32>,
    pub is_public: bool,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow,Type)]
pub struct SpaceWithUser {
    pub id: i32,
    pub user_id: i32, //created_by
    pub name: String,
    pub space_thumbnail_id: Option<i32>,
    pub is_public: bool,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub account: UserCredentialSecured,
    pub thumbnail: Option<Storage>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow,Type)]
pub struct SpaceWithUserAndThumbnail {
    pub id: i32,
    pub user_id: i32, //created_by
    pub name: String,
    pub space_thumbnail_id: Option<i32>,
    pub is_public: bool,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub account: UserCredentialSecured,
    pub thumbnail: Option<Storage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SpaceFollower {
    pub space_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct SpaceFollowerWithSpaceAndUser {
    pub space_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub space: SpaceWithUserAndThumbnail,
    pub user: UserCredentialSecured,
}