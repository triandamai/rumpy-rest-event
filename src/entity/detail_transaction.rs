use crate::dto::detail_transaction::DetailTransactionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct DetailTransaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: Option<ObjectId>,
    pub transaction_id:Option<ObjectId>,
    pub kind: String,
    pub notes: String,
    pub quantity: i64,
    pub total: f64,
    pub total_before_discount:f64,
    pub is_membership:bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl DetailTransaction {
    pub fn to_dto(self) -> DetailTransactionDTO {
        DetailTransactionDTO {
            id: self.id,
            product_id: self.product_id,
            transaction_id: self.transaction_id,
            kind: self.kind,
            notes: self.notes,
            quantity: self.quantity,
            total_before_discount: self.total_before_discount,
            total: self.total,
            is_membership: self.is_membership,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: false,
            product: None,
        }
    }
}
