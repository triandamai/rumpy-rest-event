use serde::{Deserialize,Serialize};
use validator::Validate;

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1))]
    pub product_name:String,
    pub product_description:String,
    pub product_type:String,
    pub product_price:f64,
    pub product_selling_price:f64,
    pub product_profit:f64,
    pub product_stock:i64,
}
#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1))]
    pub product_name:Option<String>,
    pub product_description:Option<String>,
    pub product_price:Option<f64>,
    pub product_selling_price:Option<f64>,
    pub product_profit:Option<f64>
}