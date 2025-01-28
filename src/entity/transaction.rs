use crate::dto::transaction_dto::TransactionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub member_id: Option<ObjectId>,
    pub notes: String,
    pub total_price: f64,
    pub total_discount: f64,
    pub kind:String,
    pub payment_method: String,
    pub payment_method_provider: Option<String>,
    pub created_by_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Transaction {
    pub fn to_dto(self) -> TransactionDTO {
        TransactionDTO {
            id: self.id,
            branch_id: self.branch_id,
            branch: None,
            member_id: self.member_id,
            member: None,
            notes: self.notes,
            kind: self.kind,
            payment_method: self.payment_method,
            payment_method_provider: self.payment_method_provider,
            total_price: self.total_price,
            total_discount: self.total_discount,
            details: None,
            created_by_id: self.created_by_id,
            created_by: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
