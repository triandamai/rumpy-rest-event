use std::sync::Arc;

use redis::Client;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::common;
use crate::common::redis::RedisClient;
use crate::common::sse::sse_emitter::SseBroadcaster;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sse: Arc<SseBroadcaster>,
    pub postgres: Pool<Postgres>,
    pub redis: RedisClient,
}

impl AppState {
    pub async fn init() -> Self {
        let env = common::env_config::EnvConfig::init();

        let sse = SseBroadcaster::create();
        let postgres = PgPoolOptions::new()
            .max_connections(10)
            .connect(env.postgres_url.as_str()).await;

        if postgres.is_err() {
            panic!("url {} -> {}", env.postgres_url, postgres.unwrap_err())
        }
        let postgres = postgres.unwrap();

        let redis = Client::open(env.redis_url.clone());
        if redis.is_err() {
            panic!("url {} -> {}", env.redis_url.clone(), redis.unwrap_err())
        }
        let redis = redis.unwrap();

        let redis_util = RedisClient::from(redis, env.mode.clone());

        AppState {
            sse,
            postgres,
            redis: redis_util,
        }
    }
}
