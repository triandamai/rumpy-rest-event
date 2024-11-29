use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::oid::ObjectId;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Product{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub product_name:String,
    pub product_description:String,
    pub product_price:i64,
    pub product_selling_price:i64,
    pub product_stock:i64,
    pub created_by:Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
