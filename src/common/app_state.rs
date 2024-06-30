use std::sync::Arc;
use redis::{Client};
use sea_orm::{Database, DatabaseConnection};
use crate::common;
use crate::common::redis_util::RedisUtil;
use crate::common::sse::sse_emitter::SseBroadcaster;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sse: Arc<SseBroadcaster>,
    pub postgres: DatabaseConnection,
    pub redis: RedisUtil,
}

impl AppState {
    pub async fn init() -> Self {
        let env = common::env_config::EnvConfig::init();

        let mut sse = SseBroadcaster::create();
        let postgres = Database::connect(env.postgres_url.clone()).await;
        if postgres.is_err() {
            panic!("url {} -> {}", env.postgres_url, postgres.unwrap_err())
        }
        let postgres = postgres.unwrap();

        let redis = Client::open(env.redis_url.clone());
        if redis.is_err() {
            panic!("url {} -> {}", env.redis_url.clone(), redis.unwrap_err())
        }
        let redis = redis.unwrap();

        let redis_util = RedisUtil::from(redis, env.mode.clone());

        AppState {
            sse,
            postgres,
            redis: redis_util,
        }
    }
}