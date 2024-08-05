use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::entity::user_credential::{UserCredential, UserCredentialSecured};

#[derive(Debug, Clone, Serialize, Deserialize, Type,PartialEq)]
#[sqlx(type_name = "conversation_type")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum ConversationType {
    Group,
    Direct,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "conversation")]
pub struct Conversation {
    pub id: i32,
    pub conversation_name: String,
    pub conversation_type: ConversationType,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationWithMember {
    pub id: i32,
    pub conversation_name: String,
    pub conversation_type: ConversationType,
    pub created_at: NaiveDateTime,
    pub members: Vec<ConversationMemberWithUser>,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversationMember {
    pub user_id: i32,
    pub joined_at: NaiveDateTime,
    pub conversation_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversationMemberWithUser {
    pub user_id: i32,
    pub joined_at: NaiveDateTime,
    pub conversation_id: i32,
    pub account: UserCredentialSecured,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversationMemberWithUserAndConversation {
    pub user_id: i32,
    pub joined_at: NaiveDateTime,
    pub conversation_id: i32,
    pub account: UserCredentialSecured,
    pub conversation: Conversation,
}