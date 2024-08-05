use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "user_credential")]
pub struct UserCredential {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub username: String,
    pub uuid: String,
    pub deleted: bool,
    pub auth_provider: AuthProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "user_credential")]
pub struct UserCredentialSecured {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub username: String,
    pub uuid: String,
    pub deleted: bool,
    pub auth_provider: AuthProvider,
}

impl UserCredentialSecured {
    pub fn from(value: UserCredential) -> Self {
        UserCredentialSecured {
            id: value.id,
            full_name: value.full_name,
            email: value.email,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
            username: value.username,
            uuid: value.uuid,
            deleted: value.deleted,
            auth_provider: value.auth_provider,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "user_status")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum UserStatus {
    Active,
    Inactive,
    WaitingConfirmation,
    Suspended,
    Locked,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "auth_provider")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum AuthProvider {
    Basic,
    Google,
    Facebook,
    Apple,
    Twitter,
}
