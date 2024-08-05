use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::entity::file::Storage;
use crate::entity::space::SpaceWithUserAndThumbnail;
use crate::entity::user_credential::UserCredentialSecured;

#[derive(Debug,Clone,Serialize,Deserialize,FromRow,Type)]
pub struct Post{
    pub id:i32,
    pub user_id:i32,
    pub space_id:Option<i32>,
    pub body:String,
    pub post_type:PostType,
    pub comments: i64,
    pub watch:i64,
    pub up_vote:i64,
    pub down_vote:i64,
    pub share_count:i64,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow,Type)]
pub struct PostWithUserAndSpace {
    pub id:i32,
    pub user_id:i32,
    pub space_id:Option<i32>,
    pub body:String,
    pub post_type:PostType,
    pub comments: i64,
    pub watch:i64,
    pub up_vote:i64,
    pub down_vote:i64,
    pub share_count:i64,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub space:Option<SpaceWithUserAndThumbnail>
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostWithUserAndSpaceAndAttachment {
    pub id:i32,
    pub user_id:i32,
    pub space_id:Option<i32>,
    pub body:String,
    pub post_type:PostType,
    pub comments: i64,
    pub watch:i64,
    pub up_vote:i64,
    pub down_vote:i64,
    pub share_count:i64,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub space:Option<SpaceWithUserAndThumbnail>,
    pub attachments: Vec<PostAttachmentWithFile>
}



#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostAttachment{
    pub post_id:i32,
    pub file_id:i32,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostAttachmentWithFile{
    pub post_id:i32,
    pub file_id:i32,
    pub created_at:NaiveDateTime,
    pub post:Post,
    pub file: Storage
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostAttachmentWithFileAndPost {
    pub post_id:i32,
    pub file_id:i32,
    pub created_at:NaiveDateTime,
    pub post:Post,
    pub file: Storage
}


#[derive(Debug,Clone,Serialize,Deserialize,FromRow,Type)]
pub struct PostComment{
    pub id:i32,
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub reply_to_id:Option<i32>,
    pub body:String,
    pub reply_count:i32,
    pub watch:i32,
    pub up_vote:i32,
    pub down_vote:i32,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow,Type)]
pub struct PostCommentWithUserAndPost{
    pub id:i32,
    pub post_id:Option<i32>,
    pub user_id:i32,
    pub reply_to_id:Option<i32>,
    pub body:String,
    pub reply_count: i64,
    pub watch:i64,
    pub up_vote:i64,
    pub down_vote:i64,
    pub created_at:NaiveDateTime,
    pub post:Option<Post>,
    pub user:UserCredentialSecured
}


#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "post_type")] // Matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // Ensures enum variants match PostgreSQL type
pub enum PostType {
    Thought,
    None
}


#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostWatch{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostWatchWithUserAndPost{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<Post>
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostUpVote{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostUpVoteWithUserAndPost{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<Post>
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostDownVote{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostDownVoteWithUserAndPost{
    pub user_id:i32,
    pub post_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<Post>
}



#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostCommentWatch{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostCommentWatchWithUserAndComment{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<PostComment>
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostCommentUpVote{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostUpVoteWithUserAndComment{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<PostComment>
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostCommentDownVote{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct PostDownVoteWithUserAndComment{
    pub user_id:i32,
    pub post_comment_id:Option<i32>,
    pub created_at:NaiveDateTime,
    pub user:UserCredentialSecured,
    pub post:Option<PostComment>
}