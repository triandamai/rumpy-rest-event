use common::i18n::I18n;
use common::mongo::get_db_name;
use common::seeder::init_seeder;
use dto::notification_dto::NotificationDTO;
use entity::notification::Notification;
use futures::StreamExt;
use log::info;
use mongodb::options::FullDocumentBeforeChangeType;
use tokio::{self, main};

use crate::common::app_state::AppState;
use crate::router::init_routes;

pub mod common;
pub mod dto;
pub mod entity;
pub mod feature;
pub mod repositories;
pub mod router;
pub mod routes;

#[main]
async fn main() -> Result<(), ()> {
    let _ = tracing_subscriber::fmt().init();
    let app_state = AppState::init().await;
    let _i18n = I18n::sync_locales(&["auth", "thread", "thread-vote", "topic"]).await;
    let _init_seeder = init_seeder(&app_state.db).await;

    let pre_image = FullDocumentBeforeChangeType::Required;

    let coll = app_state
        .db
        .database(&get_db_name())
        .collection::<NotificationDTO>("notification")
        .watch()
        .full_document_before_change(pre_image)
        .await;

    if let Ok(mut event) = coll {
        while let Some(stream) = event.next().await {
            info!(target:"event-triggered","{:?}",stream);
            if let Ok(data) = stream {
                
            }
        }
    }

    let app = init_routes(app_state);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let _app = axum::serve(tcp_listener, app).await;
    Ok(())
}
