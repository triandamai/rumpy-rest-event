use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Coach{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub full_name:String,
    pub email:String,
    pub phone_number:String,
    pub gender:String,
    pub created_by:Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
