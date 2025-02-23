use super::jwt::AuthError;
use crate::common::env_config::EnvConfig;
use crate::common::redis::RedisClient;
use crate::common::sse::sse_emitter::SseBroadcaster;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::info;
use redis::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sse: Arc<SseBroadcaster>,
    pub redis: RedisClient,
    pub postgres: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl AppState {
    pub async fn init() -> Self {
        info!(target: "app::state", "Initializing AppState");
        let env = EnvConfig::init();

        let sse = SseBroadcaster::create();

        let manager = ConnectionManager::<PgConnection>::new(env.database_url.as_str());
        let pool = Pool::builder().max_size(5).build(manager);
        if pool.is_err() {
            panic!(
                "database {} -> {:?}",
                env.database_url.clone(),
                pool.err().unwrap()
            );
        }
        let pool = pool.unwrap();
        let postgres = Arc::new(pool);

        let redis = Client::open(env.redis_url.clone());
        if redis.is_err() {
            panic!("url {} -> {}", env.redis_url.clone(), redis.unwrap_err())
        }
        let redis = redis.unwrap();

        let redis_util = RedisClient::from(redis, env.mode.clone());
        info!(target: "app::state", "Finish Initializing AppState");
        AppState {
            sse,
            redis: redis_util,
            postgres: postgres,
        }
    }
}

impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>, // <---- added this line
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state)) // <---- added this line
    }
}
