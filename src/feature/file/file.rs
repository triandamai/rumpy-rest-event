use axum::extract::State;
use axum::response::IntoResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;

pub async fn upload_file(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}

pub async fn update_file(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}

pub async fn delete_file(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {}