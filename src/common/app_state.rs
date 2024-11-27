use super::jwt::AuthError;
use crate::common::env_config::EnvConfig;
use crate::common::redis::RedisClient;
use crate::common::sse::sse_emitter::SseBroadcaster;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use mongodb::{Client as MongoClient, Database};
use redis::Client;
use std::sync::Arc;
use log::info;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sse: Arc<SseBroadcaster>,
    pub redis: RedisClient,
    pub db: Database,
}

impl AppState {
    pub async fn init() -> Self {
        info!(target: "app::state", "Initializing AppState");
        let env = EnvConfig::init();

        let sse = SseBroadcaster::create();
        let database = MongoClient::with_uri_str(env.database_url.as_str()).await;
        if database.is_err() {
            panic!("{}",database.unwrap_err());
        }
        let database = database.unwrap().database("strong-teams");
        let redis = Client::open(env.redis_url.clone());
        if redis.is_err() {
            panic!("url {} -> {}", env.redis_url.clone(), redis.unwrap_err())
        }
        let redis = redis.unwrap();

        let redis_util = RedisClient::from(redis, env.mode.clone());
        info!(target: "app::state", "Finish Initializing AppState");
        AppState {
            sse,
            db: database,
            redis: redis_util,
        }
    }
}


#[async_trait]
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
