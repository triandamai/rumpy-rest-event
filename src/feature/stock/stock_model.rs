use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UpdateStockRequest {
    #[validate(length(min="1"))]
    pub product_id: String,
    pub stock:i64
}