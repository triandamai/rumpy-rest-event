use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreateDiscountRequest {
    #[validate(length(min = 0))]
    pub title: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UpdateDiscountRequest {
    pub title: Option<String>,
    pub amount: Option<f64>,
}
