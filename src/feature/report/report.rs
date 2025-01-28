use std::collections::HashMap;

use axum::{extract::State, response::Html};

use bson::oid::ObjectId;

use crate::{
    common::{
        app_state::AppState, constant::TRANSACTION_PRODUCT, jwt::AuthContext, lang::Lang,
        middleware::Json, orm::orm::Orm,
    },
    dto::{detail_transaction::DetailTransactionDTO, transaction_dto::TransactionDTO},
    entity::product::Product,
    feature::report::report_model::ItemProductBeforeAfter,
    translate,
};

use super::report_model::{DailyReportResponse, GetDailyReportRequest, ItemCashTransaction};

pub async fn get_daily_report(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
) -> Html<String> {
    let mut html = String::from("<html><body>");
    let end_html = "<div>M</div></body></html>";
    //get all product
    let find_product = Orm::get("Product").all::<Product>(&state.db).await;
    if find_product.is_err() {
        html += end_html;
        return Html(html);
    }
    let find_product = find_product.unwrap();

    let mut product_ids: Vec<String> = Vec::new();
    let mut product_stocks: HashMap<ObjectId, ItemProductBeforeAfter> = HashMap::new();
    let mut transfer_report: HashMap<ObjectId, ItemCashTransaction> = HashMap::new();

    for product in find_product {
        if product.id.is_some() {
            if let Some(product_id) = product.id {
                let id = product_id.clone().to_string();
                product_ids.push(id.clone());
                product_stocks.insert(
                    product_id,
                    ItemProductBeforeAfter {
                        product_name: product.product_name,
                        product_stock_before: 0,
                        product_stock_after: product.product_stock,
                    },
                );
            }
        }
    }

    let find_transaction = Orm::get("transaction")
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_many("detail-transaction", "_id", "transaction_id", "details")
        .join_one("product", "detail-transaction.product_id", "_id", "product")
        .join_one("member", "member_id", "_id", "member")
        .filter_array("kind", Some("$in"), vec![TRANSACTION_PRODUCT])
        .all::<TransactionDTO>(&state.db)
        .await;

    if let Ok(transactions) = find_transaction {
        for transaction in transactions {
            if let Some(details) = transaction.details{
                for product in details {
                    
                }
            }
            
        }
    }

    //apakah transaksi harian memunculkan semua produk atau hanya spesifik produk pada hari tersebut?
    let mut build_stock = r#"<div>
    <p>Stock Opname Produk</p>
    <table><thead><tr><td>No</td><td>Name</td><td>Before</td><td>After</td></tr></thead><tbody>
    "#
    .to_string();

    for stock in product_stocks {
        let mut tr = "<tr>".to_string();
        tr += format!("<td>{}</td>", stock.1.product_name).as_str();
        tr += format!("<td>{}</td>", stock.1.product_stock_before).as_str();
        tr += format!("<td>{}</td>", stock.1.product_stock_after).as_str();
        tr += "</tr>"
    }
    build_stock += "</tbody></div></table>";
    html += &build_stock;

    Html(html)
}
