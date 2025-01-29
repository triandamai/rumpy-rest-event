use std::collections::HashMap;

use axum::extract::State;
use bson::DateTime;
use chrono::{Datelike, Utc};
use dashboard::{ChartTrend, DashboardStatResponse};
use log::info;

use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, jwt::AuthContext, lang::Lang, orm::orm::Orm,
    },
    dto::{product_dto::ProductDTO, transaction_dto::TransactionDTO},
    entity::{coach::Coach, member::Member, product::Product},
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
            stats.from_date = from_date.clone().unwrap();
            stats.to_date = to_date.clone().unwrap();
            let find_transaction = Orm::get("transaction")
                .filter_object_id("branch_id", &branch_id)
                .and()
                .filter_between_date("created_at", from_date.unwrap(), to_date.unwrap())
                .join_many("detail-transaction", "_id", "transaction_id", "details")
                .all::<TransactionDTO>(&state.db)
                .await;

            let mut trend_non_member: HashMap<String, ChartTrend> = HashMap::new();
            let mut trend_member: HashMap<String, ChartTrend> = HashMap::new();
            if let Ok(transactions) = find_transaction {
                for transaction in transactions {
                    let created_at = transaction.created_at;
                    let datetime =
                        chrono::DateTime::from_timestamp_millis(created_at.timestamp_millis());
                    if datetime.is_some() {
                        let datetime = datetime.unwrap();
                        let axis = datetime.format("%Y-%m-%d").to_string();
                        info!(target:"Sasa","{}",axis);
                        if let Some(details) = transaction.details {
                            for detail in details {
                                if detail.is_membership {
                                    stats.total_membership_trend += detail.total;
                                    match trend_member.get(&axis.clone()) {
                                        Some(axis_trend) => {
                                            let mut clone_axis = axis_trend.clone();
                                            clone_axis.value += detail.total;
                                            trend_member.insert(axis.clone(), clone_axis);
                                        }
                                        None => {
                                            trend_member.insert(
                                                axis.clone(),
                                                ChartTrend {
                                                    datetime: axis.clone(),
                                                    value: detail.total,
                                                },
                                            );
                                        }
                                    }
                                } else {
                                    stats.total_non_membership_trend += detail.total;
                                    match trend_non_member.get(&axis.clone()) {
                                        Some(axis_trend) => {
                                            let mut clone_axis = axis_trend.clone();
                                            clone_axis.value += detail.total;
                                            trend_non_member.insert(axis.clone(), clone_axis);
                                        }
                                        None => {
                                            trend_non_member.insert(
                                                axis.clone(),
                                                ChartTrend {
                                                    datetime: axis.clone(),
                                                    value: detail.total,
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
            for (_axis, value) in trend_member {
                stats.membership_trend.push(value);
            }
            for (_axis, value) in trend_non_member {
                stats.non_membership_trend.push(value);
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
    }
    return ApiResponse::ok(stats, translate!("dashboard.success", lang).as_str());
}
