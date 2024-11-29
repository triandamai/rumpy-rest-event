use axum::extract::{Query, State};
use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::entity::coach::Coach;

pub async fn get_list_coach(
    state:State<AppState>,
    lang:Lang,
    auth_context: AuthContext,
    query:Query<PaginationRequest>
)->ApiResponse<PagingResponse<Coach>>{
    ApiResponse::failed("")
}

pub async fn create_coach(
    state:State<AppState>,
    lang:Lang,
    auth_context: AuthContext,
)->ApiResponse<Coach>{
    ApiResponse::failed("")
}