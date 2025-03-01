use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreatedThreadRequest {
    pub quote_thread_id: Option<String>,
    pub reply_to_thread_id: Option<String>,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub attachment: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateThreadRequest {
    pub quote_thread_id: Option<String>,
    #[validate(length(min = 1))]
    pub slug: Option<String>,
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
    pub new_attachment: Vec<String>,
    pub removed_attachment: Vec<String>,
}
