use tokio::{self, main};

use crate::common::app_state::AppState;
use crate::router::init_routes;

pub mod common;
pub mod entity;
pub mod feature;
pub mod router;
pub mod routes;
pub mod repositories;

#[main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let app_state = AppState::init().await;

    let app = init_routes(app_state);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
