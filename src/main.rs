use tokio::{self, main};

use crate::common::app_state::AppState;
use crate::common::i18n::I18n;
use crate::common::seeder::init_seeder;
use crate::router::init_routes;


pub mod common;
pub mod entity;
pub mod feature;
pub mod router;
pub mod routes;
pub mod repositories;
pub mod dto;

#[main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let app_state = AppState::init().await;
    let _init_i18n = I18n::load_locales().await;
    let _init_seeder = init_seeder(
        &app_state.db
    ).await;

    let app = init_routes(app_state);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let _app = axum::serve(tcp_listener, app).await;

    Ok(())
}
