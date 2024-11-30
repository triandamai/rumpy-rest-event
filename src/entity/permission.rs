use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::oid::ObjectId;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub value: String,
    pub name: String,
    pub group: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
