use std::collections::HashMap;

use axum::{extract::State, response::Html};

use bson::oid::ObjectId;

use crate::{
    common::{
        app_state::AppState,
        constant::{PAYMENT_METHOD_CASH, PAYMENT_METHOD_TRANSFER},
        jwt::AuthContext,
        lang::Lang,
        orm::orm::Orm,
    },
    dto::transaction_dto::TransactionDTO,
    entity::product::Product,
    feature::report::report_model::ItemProductBeforeAfter,
};

use super::report_model::ItemReportTransaction;

pub async fn get_daily_report(
    state: State<AppState>,
    // auth_context: AuthContext,
    lang: Lang,
) -> Html<String> {
    let mut html = String::from(
        r#"
        <!doctype html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <script src="https://unpkg.com/@tailwindcss/browser@4"></script>
              <style type="text/tailwindcss">
      @theme {
        --color-clifford: #da373d;
      }
    </style>
        </head>
        <body>
        <h1 class="text-3xl font-bold underline text-clifford">Hello world!</h1>
        "#,
    );
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
    let mut transfer_report: HashMap<ObjectId, ItemReportTransaction> = HashMap::new();
    let mut cash_report: HashMap<ObjectId, ItemReportTransaction> = HashMap::new();

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
        // .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_many("detail-transaction", "_id", "transaction_id", "details")
        .join_nested_one(
            "product",
            "detail-transaction.product_id",
            "_id",
            "product",
            "detail-transaction.product",
        )
        .join_nested_one(
            "membership",
            "detail-transaction.product_id",
            "_id",
            "membership",
            "detail-transaction.membership",
        )
        .join_one("member", "member_id", "_id", "member")
        .all::<TransactionDTO>(&state.db)
        .await;

    if let Ok(transactions) = find_transaction {
        for transaction in transactions {
            let method = transaction.payment_method;
            let default_string = String::new();
            let coach_name = match transaction.member {
                Some(m) => match m.coach {
                    Some(c) => c.full_name,
                    None => default_string.clone(),
                },
                None => default_string.clone(),
            };

            if let Some(details) = transaction.details {
                for item in details {
                    if let Some(item_product) = item.product {
                        let id = item_product.id.unwrap();
                        if method == PAYMENT_METHOD_CASH {
                            match cash_report.get(&id) {
                                Some(temp) => {
                                    let mut t = temp.clone();
                                    t.amount += item.total;
                                    cash_report.insert(id, t);
                                }
                                None => {
                                    cash_report.insert(
                                        id,
                                        ItemReportTransaction {
                                            product_name: item_product.product_name.clone(),
                                            coach_name: coach_name.clone(),
                                            amount: item.total,
                                            description: default_string.clone(),
                                        },
                                    );
                                }
                            }
                        }

                        if method == PAYMENT_METHOD_TRANSFER {
                            match transfer_report.get(&id) {
                                Some(temp) => {
                                    let mut t = temp.clone();
                                    t.amount += item.total;
                                    cash_report.insert(id, t);
                                }
                                None => {
                                    transfer_report.insert(
                                        id,
                                        ItemReportTransaction {
                                            product_name: item_product.product_name.clone(),
                                            coach_name: coach_name.clone(),
                                            amount: item.total,
                                            description: transaction
                                                .payment_method_provider
                                                .clone()
                                                .unwrap_or(default_string.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    if let Some(item_membership) = item.membership {
                        let id = item_membership.id.unwrap();
                        if method == PAYMENT_METHOD_CASH {
                            match cash_report.get(&id) {
                                Some(temp) => {
                                    let mut t = temp.clone();
                                    t.amount += item.total;
                                    cash_report.insert(id, t);
                                }
                                None => {
                                    cash_report.insert(
                                        id,
                                        ItemReportTransaction {
                                            product_name: item_membership.name.clone(),
                                            coach_name: coach_name.clone(),
                                            amount: item.total,
                                            description: default_string.clone(),
                                        },
                                    );
                                }
                            }
                        }

                        if method == PAYMENT_METHOD_TRANSFER {
                            match transfer_report.get(&id) {
                                Some(temp) => {
                                    let mut t = temp.clone();
                                    t.amount += item.total;
                                    cash_report.insert(id, t);
                                }
                                None => {
                                    transfer_report.insert(
                                        id,
                                        ItemReportTransaction {
                                            product_name: item_membership.name.clone(),
                                            coach_name: coach_name.clone(),
                                            amount: item.total,
                                            description: transaction
                                                .payment_method_provider
                                                .clone()
                                                .unwrap_or(default_string.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //
    let mut build_payment = r#"
  <div>
    <p>Cash</p>
    <table>
        <thead>
            <tr>
                <td>No</td>
                <td>Name</td>
                <td>Before</td>
                <td>After</td>
            </tr>
        </thead>
    <tbody>
    "#
    .to_string();
    for (_, transfer) in cash_report {
        let mut tr = "<tr>".to_string();
        tr += format!("<td>{}</td>", transfer.product_name).as_str();
        tr += format!("<td>{}</td>", transfer.description).as_str();
        tr += format!("<td>{}</td>", transfer.amount).as_str();
        tr += "</tr>"
    }

    build_payment += r#"</tbody></table></div>"#;
    html += &build_payment;
    //

    let mut build_payment = r#"
    <div>
      <p>Transfer</p>
      <table>
          <thead>
              <tr>
                  <td>No</td>
                  <td>Name</td>
                  <td>Before</td>
                  <td>After</td>
              </tr>
          </thead>
      <tbody>
      "#
    .to_string();
    for (_, transfer) in transfer_report {
        let mut tr = "<tr>".to_string();
        tr += format!("<td>{}</td>", transfer.product_name).as_str();
        tr += format!("<td>{}</td>", transfer.description).as_str();
        tr += format!("<td>{}</td>", transfer.amount).as_str();
        tr += "</tr>"
    }

    build_payment += r#"</tbody></table></div></div>"#;
    html += &build_payment;

    //apakah transaksi harian memunculkan semua produk atau hanya spesifik produk pada hari tersebut?
    let mut build_stock = r#"
    <div>
    <p>Stock Opname Produk</p>
    <table>
        <thead>
            <tr>
                <td>No</td>
                <td>Name</td>
                <td>Before</td>
                <td>After</td>
            </tr>
        </thead>
    <tbody>
    "#
    .to_string();
    for stock in product_stocks {
        let mut tr = "<tr>".to_string();
        tr += format!("<td>{}</td>", stock.1.product_name).as_str();
        tr += format!("<td>{}</td>", stock.1.product_stock_before).as_str();
        tr += format!("<td>{}</td>", stock.1.product_stock_after).as_str();
        tr += "</tr>"
    }
    build_stock += r#"
    </tbody>
    </div>
    </table>
    "#;
    html += &build_stock;

    Html(html)
}


