use crate::dto::detail_transaction::DetailTransactionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailTransaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: Option<ObjectId>,
    pub kind: String,
    pub notes: String,
    pub quantity: i64,
    pub total: f64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl DetailTransaction {
    pub fn to_dto(self) -> DetailTransactionDTO {
        DetailTransactionDTO {
            id: self.id,
            product_id: self.product_id,
            kind: self.kind,
            notes: self.notes,
            quantity: self.quantity,
            total: self.total,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: false,
        }
    }
}
