use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use crate::entity::user_credential::{UserCredential, UserCredentialSecured};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "friend")]
pub struct Friend {
    pub friend_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FriendWithDetail {
    pub friend_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub friend: UserCredentialSecured,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[sqlx(type_name = "friend")]
pub struct FriendRequest {
    pub id: i32,
    pub from_id: i32,
    pub friend_id: i32,
    pub is_rejected: bool,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[sqlx(type_name = "friend")]
pub struct FriendRequestWithDetail {
    pub id: i32,
    pub from_id: i32,
    pub friend_id: i32,
    pub is_rejected: bool,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub friend: UserCredentialSecured,
}