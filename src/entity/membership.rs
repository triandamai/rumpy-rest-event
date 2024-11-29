use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Membership{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub name:String,
    pub price:i64,
    pub quota:i64,
    pub created_by:Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
