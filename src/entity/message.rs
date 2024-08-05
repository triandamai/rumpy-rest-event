use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "message_type")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum MessageType {
    Text,
    Sticker,
    Image,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub conversation_id: i32,
    pub message_content: String,
    pub message_type: MessageType,
    pub message_sent_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageWithAttachment {
    pub id: i32,
    pub sender_id: i32,
    pub conversation_id: i32,
    pub message_content: String,
    pub message_type: MessageType,
    pub message_sent_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub attachment: Option<MessageAttachment>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "attachment_type")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum AttachmentType {
    Image,
    Video,
    File,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "message_attachment")]
pub struct MessageAttachment {
    pub id: i32,
    pub message_id: i32,
    pub attachment_type: AttachmentType,
    pub attachment_url: String,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "status_message")] // Ensure this matches your PostgreSQL enum type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
enum StatusMessage {
    Sent,
    Delivered,
    Read,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
struct MessageStatus {
    message_id: i32,
    user_id: i32,
    conversation_id: i32,
    message_status: StatusMessage,
    created_at: NaiveDateTime,
}