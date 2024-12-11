use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::dto::transaction_dto::TransactionDTO;

#[derive(Debug,Serialize,Deserialize)]
pub struct Transaction{
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id:Option<ObjectId>,
    pub branch_id:Option<ObjectId>,
    pub member_id:Option<ObjectId>,
    pub total_price_before_discount:i64,
    pub notes:String,
    pub total_price:i64,
    pub total_discount:f64,
    pub is_membership:bool,
    pub created_by_id:Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted:bool,
}

impl Transaction{
    pub fn to_dto(self)->TransactionDTO{
        TransactionDTO{
            id: self.id,
            branch_id: self.branch_id,
            member_id: self.member_id,
            total_price_before_discount: self.total_price_before_discount,
            notes: self.notes,
            total_price: self.total_price,
            total_discount: self.total_discount,
            is_membership: self.is_membership,
            created_by: self.created_by_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}