use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::oid::ObjectId;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Transaction{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub member_id:Option<ObjectId>,
    pub total_price_before_discount:i64,
    pub notes:String,
    pub total_price:i64,
    pub total_discount:f64,
    pub is_membership:bool,
    pub created_by:Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
