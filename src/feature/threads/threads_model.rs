use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct FilterThreadRequest {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub q:Option<String>,
    pub mentions:Option<Vec<String>>,
    pub tags:Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateNewThreadRequest {
    pub quote_thread_id: Option<String>,
    #[validate(length(min = 1))]
    pub title: String,
    pub content: String,
    pub mentions:Option<Vec<String>>,
    pub tags:Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateThreadRequest {
    pub quote_thread_id: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub content: String,
    pub mentions:Option<Vec<String>>,
    pub tags:Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1))]
    pub thread_id: String,
    pub content: String,
    pub comment_id: Option<String>,
    pub tags:Option<Vec<String>>,
    pub mentions:Option<Vec<String>>,
}


#[derive(Deserialize, Serialize, Validate)]
pub struct CreateVoteRequest {
    #[validate(length(min = 1))]
    pub ref_id: String,
    pub vote_type: String,
}
