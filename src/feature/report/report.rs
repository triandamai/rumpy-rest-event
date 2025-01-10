use axum::extract::State;

use crate::common::{api_response::ApiResponse, app_state::AppState, middleware::Json};

use super::report_model::{DailyReportResponse, GetDailyReportRequest};

async fn get_daily_report(
    state: State<AppState>,
    body: Json<GetDailyReportRequest>,
) -> ApiResponse<DailyReportResponse> {

    //get all product


    //get transaction by product and date

    //apakah transaksi harian memunculkan semua produk atau hanya spesifik produk pada hari tersebut?
    

    ApiResponse::bad_request("")
}
