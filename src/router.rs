use axum::Router;
use axum::routing::{get, post};
use crate::common::app_state::AppState;
use crate::routes;
use crate::feature;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/sse/register", get(feature::stream::register_sse))
        .route("/sse/send-to-channel", post(feature::stream::send_to_channel))
        .route("/sse/send-to-user", post(feature::stream::send_to_user))
        .route("/sse/broadcast", post(feature::stream::send_broadcast))
        .with_state(state)
}