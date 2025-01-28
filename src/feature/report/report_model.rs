use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemProductBeforeAfter {
    pub product_name: String,
    pub product_stock_before: i64,
    pub product_stock_after: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCashTransaction {
    pub product_name: String,
    pub coach_name: String,
    pub product_stock_before: i64,
    pub product_stock_after: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemTransferTransaction {
    pub product_name: String,
    pub description: String,
    pub product_stock_before: i64,
    pub product_stock_after: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDailyReportRequest {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyReportResponse {}
