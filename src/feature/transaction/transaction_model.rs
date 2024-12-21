use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct CreateTransactionTopUpRequest {
    pub member_id: String,
    pub amount: f64,
    pub notes: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct DetailTransactionRequest {
    pub product_id: String,
    pub total: f64,
    pub discount:f64,
    pub quantity:i64,
    pub notes:Option<String>
}


#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct CreateTransactionMembershipProductRequest {
    pub member_id: String,
    pub total: f64,
    pub notes: Option<String>,
    pub products:Vec<DetailTransactionRequest>
}
