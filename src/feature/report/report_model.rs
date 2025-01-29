use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemProductBeforeAfter {
    pub product_name: String,
    pub product_stock_before: i64,
    pub product_stock_after: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemReportTransaction {
    pub product_name: String,
    pub coach_name: String,
    pub amount: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDailyReportRequest {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyReportResponse {}
