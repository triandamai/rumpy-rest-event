use crate::dto::product_dto::ProductDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub product_name: String,
    pub product_description: String,
    pub product_price: i64,
    pub product_selling_price: i64,
    pub product_stock: i64,
    pub created_by: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Product {
    pub fn to_dto(self) -> ProductDTO {
        ProductDTO {
            id: self.id,
            branch_id: self.branch_id,
            product_name: self.product_name,
            product_description: self.product_description,
            product_price: self.product_price,
            product_selling_price: self.product_selling_price,
            product_stock: self.product_stock,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
