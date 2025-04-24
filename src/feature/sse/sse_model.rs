use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterSse {
    #[validate(length(min = 1))]
    pub device_id: String,
    #[validate(length(min = 1))]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendToUserRequest {
    #[validate(length(min = 1))]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub device_id: String,
    #[validate(length(min = 1))]
    pub event_name: String,
    #[validate(length(min = 1))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendToUserChannel {
    #[validate(length(min = 1))]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub event_name: String,
    #[validate(length(min = 1))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendBroadcastRequest {
    #[validate(length(min = 1))]
    pub event_name: String,
    #[validate(length(min = 1))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendMessageResponse {
    #[validate(length(min = 1))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SubscribeToTopicRequest {
    #[validate(length(min = 1))]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEventMessagePayload {
    pub ref_id: String,
    pub context: serde_json::Value,
    pub data: serde_json::Value,
}

pub const SSE_EVENT_MENTIONED: &str = "event_thread_mentioned";
pub const SSE_EVENT_REPLY: &str = "event_thread_reply";
pub const SSE_EVENT_QUOTE: &str = "event_thread_quote";
