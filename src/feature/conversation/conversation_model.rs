use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::entity::conversation;
use crate::entity::conversation::{ConversationMember, ConversationWithMember};
use crate::entity::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePagination {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateToken {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDirectConversationRequest {
    pub conversation_name: String,
    pub members: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    pub conversation: conversation::Conversation,
    pub members: Vec<ConversationMember>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextChatRequest {
    pub content: String,
    pub conversation_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub conversation: ConversationWithMember,
    pub message: Message,
}
