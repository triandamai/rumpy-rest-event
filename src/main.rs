use std::str::FromStr;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use tokio::{self, main};

use migration::{MigrationTrait, Migrator, MigratorTrait};

use crate::common::app_state::AppState;
use crate::entity::sea_orm_active_enums::{AuthProvider, UserStatus};
use crate::router::init_routes;

pub mod common;
pub mod entity;
pub mod feature;
pub mod router;
pub mod routes;

#[main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let app_state = AppState::init().await;

    let _ = Migrator::up(&app_state.postgres, None).await;
    let _ = Migrator::down(&app_state.postgres, None).await;
    let _ = seeding_data(&app_state.postgres).await;

    let app = init_routes(app_state);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}

async fn seeding_data(conn: &DatabaseConnection) {
    let uuid = uuid::Uuid::from_str("1843ab6a-d56b-4427-9f84-221dcc1b582a");
    if uuid.is_ok() {
        let uuid = uuid.unwrap();
        let exist = entity::user_credential::Entity::find_by_id(uuid)
            .one(conn)
            .await;
        if exist.is_ok() {
            let exist = exist.unwrap();
            if exist.is_none() {
                let model = entity::user_credential::ActiveModel {
                    id: Set(uuid),
                    username: Set("triandamai".to_string()),
                    password: Set(
                        "$2a$12$R0EIvnvgqZe12Gc8C3xQzu313ouJX.CsAJ6d8jZDPsTLEmBjAf6j2".to_string(),
                    ),
                    full_name: Set("Trian Damai".to_string()),
                    email: Set("triandamai@gmail.com".to_string()),
                    auth_provider: Set(AuthProvider::Basic),
                    user_status: Set(UserStatus::Active),
                    ..Default::default()
                };

                let _ = model.insert(conn).await;
            }
        }
    }
}
