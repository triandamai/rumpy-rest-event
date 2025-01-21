use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMembershipRequest {
    #[validate(length(min = 0))]
    pub name: String,
    pub price: f64,
    pub price_per_item: f64,
    pub quota: i64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateMembershipRequest {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub price_per_item: Option<f64>,
    pub quota: Option<i64>,
    pub description: Option<String>,
}
