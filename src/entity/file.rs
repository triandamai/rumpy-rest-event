use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug,Clone,Serialize,Deserialize,FromRow,Type)]
pub struct Storage {
    pub id:i32,
    pub file_name:String,
    pub mime_type:String,
    pub is_user:bool,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime
}