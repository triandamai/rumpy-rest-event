use crate::dto::discount_dto::DiscountDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Discount {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub amount: f64,
    pub created_by: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Discount {
    pub fn to_dto(self) -> DiscountDTO {
        DiscountDTO {
            id: self.id,
            title: self.title,
            amount: self.amount,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted
        }
    }
}
