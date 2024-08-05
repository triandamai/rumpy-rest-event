use axum::extract::{Query, State};
use axum::response::IntoResponse;
use crate::common::api_response::PaginationRequest;
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;

pub async fn get_list_friend(
    state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {}

pub async fn send_friend_request(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}

pub async fn cancel_friend_request(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}

pub async fn accept_friend_request(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}

pub async fn reject_friend_request(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}