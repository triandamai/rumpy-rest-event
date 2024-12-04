use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MemberLog{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub member_id:Option<ObjectId>,
    pub name:String,
    pub value:String,
    pub created_at:DateTime,
    pub updated_at:DateTime,
}