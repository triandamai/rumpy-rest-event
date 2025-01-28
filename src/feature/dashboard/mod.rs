use axum::extract::State;
use bson::DateTime;
use chrono::{Datelike, Utc};
use dashboard::{ChartTrend, DashboardStatResponse};

use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, jwt::AuthContext, lang::Lang, orm::orm::Orm,
    },
    dto::{product_dto::ProductDTO, transaction_dto::TransactionDTO},
    entity::{coach::Coach, member::Member, product::Product, transaction::Transaction},
    translate,
};

pub mod dashboard;

pub async fn get_dashboard_stat(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
) -> ApiResponse<DashboardStatResponse> {
    let branch = auth_context.branch_id;
    let mut stats = DashboardStatResponse::new();
    if let Some(branch_id) = branch {
        let find_total_coach = Orm::get("coach")
            .filter_object_id("branch_id", &branch_id)
            .all::<Coach>(&state.db)
            .await;
        if find_total_coach.is_ok() {
            stats.total_coach = find_total_coach.unwrap().len() as i64;
        }

        let find_total_member = Orm::get("member")
            .filter_object_id("branch_id", &branch_id)
            .all::<Member>(&state.db)
            .await;
        if find_total_member.is_ok() {
            stats.total_member = find_total_member.unwrap().len() as i64;
        }

        let find_product = Orm::get("product")
            .filter_object_id("branch_id", &branch_id)
            .group_by_asc("product_stock")
            .all::<Product>(&state.db)
            .await;

        if let Ok(products) = find_product {
            for product in products {
                if product.is_membership {
                    stats.total_membership_product += 1
                } else {
                    stats.total_non_membership_product += 1
                }
            }
        }

        let current_date = Utc::now();
        let to_date = DateTime::builder()
            .year(current_date.year())
            .month(current_date.month() as u8)
            .day(current_date.day() as u8)
            .build();

        let from_date = DateTime::builder()
            .year(current_date.year())
            .month(current_date.month() as u8)
            .day(1)
            .build();

        if to_date.is_ok() && from_date.is_ok() {
            let find_transaction = Orm::get("transaction")
                .filter_object_id("branch_id", &branch_id)
                .and()
                .filter_between_date("created_at", from_date.unwrap(), to_date.unwrap())
                .join_many("detail-transaction", "_id", "transaction_id", "details")
                .all::<TransactionDTO>(&state.db)
                .await;

            let mut trend_non_member: Vec<ChartTrend> = Vec::new();
            let mut trend_member: Vec<ChartTrend> = Vec::new();
            if let Ok(transactions) = find_transaction {
                for transaction in transactions {
                    let mut total_non_member: f64 = 0.0;
                    let mut total_member: f64 = 0.0;
                    if let Some(details) = transaction.details {
                        for detail in details {
                            if detail.is_membership {
                                total_member += detail.total;
                            } else {
                                total_non_member += detail.total;
                            }
                        }
                    }
                    trend_non_member.push(ChartTrend {
                        date: transaction.created_at,
                        value: total_non_member,
                    });
                    trend_member.push(ChartTrend {
                        date: transaction.created_at,
                        value: total_member,
                    });
                }
            }
        }

        let find_product = Orm::get("product")
            .join_one("file-attachement", "_id", "ref_id", "product_image")
            .filter_number("product_stoc", Some("$lte"), 10)
            .all::<ProductDTO>(&state.db)
            .await;
        
        if let Ok(products) = find_product {
            stats.stock = products
        }
    }

    return ApiResponse::ok(stats, translate!("dashboard.success", lang).as_str());
}
