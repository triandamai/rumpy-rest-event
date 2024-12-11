use crate::dto::product_log_dto::ProductLogDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub description: String,
    pub log_type: String,
    pub stock: i64,
    pub created_by: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Product {
    pub fn to_dto(self) -> ProductLogDTO {
        ProductLogDTO {
            id: self.id,
            branch_id: self.branch_id,
            description: self.description,
            log_type: self.log_type,
            stock: self.stock,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
