use crate::dto::member_cart::MemberCartDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberCart {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub member_id: Option<ObjectId>,
    pub product_id: Option<ObjectId>,
    pub notes: String,
    pub quantity: i64,
    pub discount: f64,
    pub total: f64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl MemberCart {
    pub fn to_dto(self) -> MemberCartDTO {
        MemberCartDTO {
            id: self.id,
            member_id: self.member_id,
            member: None,
            notes: self.notes,
            product_id: self.product_id,
            total: self.total,
            product: None,
            quantity: self.quantity,
            discount: self.discount,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
