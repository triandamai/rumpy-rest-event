use serde::{Deserialize, Serialize};
use crate::entity::post::PostType;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CreatePostRequest{
    pub space_id:Option<i32>,
    pub body:String,
    pub post_type:PostType,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CreatePostCommentRequest{
    pub post_id:Option<i32>,
    pub body:String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CreateReplyPostCommentRequest{
    pub post_id:Option<i32>,
    pub comment_id:Option<i32>,
    pub body:String,
}