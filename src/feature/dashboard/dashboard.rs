use crate::common::bson::serialize_datetime;
use bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::dto::product_dto::ProductDTO;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChartTrend {
    pub datetime: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DashboardStatResponse {
    pub total_member: i64,
    pub total_coach: i64,
    pub total_membership_product: i64,
    pub total_non_membership_product: i64,
    #[serde(serialize_with = "serialize_datetime")]
    pub from_date: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub to_date: DateTime,
    pub total_membership_trend: f64,
    pub total_non_membership_trend: f64,
    pub non_membership_trend: Vec<ChartTrend>,
    pub membership_trend: Vec<ChartTrend>,
    pub stock: Vec<ProductDTO>,
}

impl DashboardStatResponse {
    pub fn new() -> Self {
        DashboardStatResponse {
            total_member: 0,
            total_coach: 0,
            total_membership_product: 0,
            total_non_membership_product: 0,
            non_membership_trend: vec![],
            membership_trend: vec![],
            stock: vec![],
            from_date: DateTime::now(),
            to_date: DateTime::now(),
            total_membership_trend: 0.0,
            total_non_membership_trend: 0.0,
        }
    }
}
