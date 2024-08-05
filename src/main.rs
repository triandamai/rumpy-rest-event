use sqlx::{ Executor, Pool, Postgres};
use tokio::{self, main};

use crate::common::app_state::AppState;
use crate::entity::conversation::{Conversation };
use crate::router::init_routes;

pub mod common;
pub mod entity;
pub mod feature;
pub mod router;
pub mod routes;
pub mod seeder;
pub mod repositories;

#[main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let app_state = AppState::init().await;

    migrate(&app_state.postgres).await;
    seeder::seed(&app_state.postgres).await;

    let app = init_routes(app_state);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}


async fn migrate(
    pool: &Pool<Postgres>
) {
    let _ = pool.execute(include_str!("../migrations/20240705123308_first_initial.sql"))
        .await
        .expect("Failed to initialis scheme");

    let s = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT DISTINCT conversation.* FROM conversation_member as member
        LEFT JOIN conversation ON conversation.id = member.conversation_id
        WHERE member.user_id = $1
        ORDER BY conversation.id
        "#
    )
        .bind(1)
        .fetch_all(pool)
        .await;

}
//
// fn row_to_json(row: &PgRow) -> HashMap<String, String> {
//     let mut result = HashMap::new();
//     for col in row.columns() {
//         let value = row.try_get_raw(col.ordinal()).unwrap();
//         // let value = match value.is_null() {
//         //     true => "NULL".to_string(),
//         //     false => value.as_str().unwrap().to_string(),
//         // };
//         result.insert(
//             col.name().to_string(),
//             value.type_info().to_string(),
//         );
//     }
//
//     result
// }