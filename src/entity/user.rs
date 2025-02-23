use crate::schema::tb_user;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use diesel::prelude::Insertable;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name=tb_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    // pub password: Option<String>,
    pub app_meta_data: Option<serde_json::Value>,
    pub user_meta_data: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub confirmation_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=tb_user)]
pub struct CreateUser {
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub app_meta_data: Option<serde_json::Value>,
    pub user_meta_data: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub confirmation_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
}
