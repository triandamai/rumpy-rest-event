use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfilePicture {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub mime_type: String,
    pub file_name: String,
    pub bucket_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
