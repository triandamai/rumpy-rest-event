use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DetailTransaction{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub product_id:Option<ObjectId>,
    pub notes:String,
    pub quantity:i64,
    pub total:f64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
