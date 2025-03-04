use axum::extract::{Query, State};
use crate::common::api_response::{ApiResponse, PaginationRequest};
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::dto::topic_dto::TopicDTO;
use crate::i18n;

pub mod topic_model;

pub async fn get_list_topic(
    state: State<AppState>,
    lang:Lang,
    Query(query):Query<PaginationRequest>
)->ApiResponse<String>{
    let i18n = i18n!("topic",lang);

    ApiResponse::failed("")
}

pub async fn create_topic()->ApiResponse<TopicDTO>{
    ApiResponse::failed("")
}