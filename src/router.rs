use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::middleware::method_not_allowed;
use crate::{feature, routes};
use axum::routing::{get, post};
use axum::Router;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/auth/sign-up-email", post(feature::auth::sign_up_email))
        .layer(axum::middleware::from_fn(method_not_allowed))
        .fallback(handle_404)
        .with_state(state)
}

async fn handle_404() -> ApiResponse<String> {
    ApiResponse::not_found("Not found.")
}
