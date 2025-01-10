use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDailyReportRequest {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyReportResponse {}
