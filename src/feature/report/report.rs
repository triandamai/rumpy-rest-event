use axum::extract::State;

use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, lang::Lang, middleware::Json, orm::orm::Orm,
    },
    entity::{detail_transaction::DetailTransaction, product::Product},
    translate,
};

use super::report_model::{DailyReportResponse, GetDailyReportRequest};

async fn get_daily_report(
    state: State<AppState>,
    lang: Lang,
    body: Json<GetDailyReportRequest>,
) -> ApiResponse<DailyReportResponse> {
    //get all product
    let find_product = Orm::get("Product").all::<Product>(&state.db).await;
    if find_product.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    let find_product = find_product.unwrap();

    let mut product_ids: Vec<String> = Vec::new();

    for product in find_product {
        if product.id.is_some() {
            let id = product.id.unwrap().to_string();
            product_ids.push(id);
        }
    }

    let find_transaction_by_product = Orm::get("detail-transaction")
        .and()
        .filter_array::<String>("product_id", Some("$in"), product_ids)
        .filter_string("created_at", Some(""), "")
        .all::<DetailTransaction>(&state.db)
        .await;
    if find_transaction_by_product.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    //apakah transaksi harian memunculkan semua produk atau hanya spesifik produk pada hari tersebut?

    ApiResponse::bad_request("")
}
