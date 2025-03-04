use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UserProfile {
    pub id:Option<ObjectId>,
    pub bio: Option<String>,
    pub websites: Option<Vec<String>>,
    pub follower:Option<i64>,
    pub following:Option<i64>,
    pub created_at:DateTime,
    pub updated_at:DateTime,
    pub deleted:bool,
}
