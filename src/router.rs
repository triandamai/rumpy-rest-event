use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::feature;
use crate::routes;
use axum::routing::{get, post};
use axum::Router;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/sync-locales", get(routes::index::generate_locales))
        .route("/test-locales", get(routes::index::test_locales))
        .route("/sse/list", get(feature::sse::sse::get_active_subscriber))
        .route("/sse/register", get(feature::sse::sse::register_sse))
        .route("/sse/send-to-user", post(feature::sse::sse::send_to_user))
        .route(
            "/sse/send-to-user-device",
            post(feature::sse::sse::send_to_user_device),
        )
        .route("/sse/broadcast", post(feature::sse::sse::send_broadcast))
        .route(
            "/sse/subscribe-topic",
            post(feature::sse::sse::subscribe_to_topic),
        )
        .route(
            "/sse/unsubscribe-topic",
            post(feature::sse::sse::unsubscribe_to_topic),
        )
        //auth
        .route(
            "/auth/sign-in",
            post(feature::auth::auth::sign_in),
        )
        .route(
            "/auth/sign-out",
            post(feature::auth::auth::sign_out),
        )
        //permission
        .route(
            "/permission/assign",
            post(feature::permission::permission::assign_permission),
        )
        //branch
        .route(
            "/branch/list",
            get(feature::branch::branch::get_list_branch),
        )
        .route("/branch", post(feature::branch::branch::create_branch))
        .fallback(handle_404)
        .with_state(state)
}

async fn handle_404() -> ApiResponse<String> {
    ApiResponse::not_found("Not found.")
}
