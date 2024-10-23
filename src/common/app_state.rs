use crate::common::env_config::EnvConfig;
use crate::common::redis::RedisClient;
use crate::common::sse::sse_emitter::SseBroadcaster;
use mongodb::{Client as MongoClient, Database, IndexModel};
use redis::Client;
use std::sync::Arc;
use bson::doc;
use mongodb::options::IndexOptions;
use crate::entity::thread::Thread;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sse: Arc<SseBroadcaster>,
    pub redis: RedisClient,
    pub db: Database,
}

impl AppState {
    pub async fn init() -> Self {
        let env = EnvConfig::init();

        let sse = SseBroadcaster::create();
        let database = MongoClient::with_uri_str(env.database_url.as_str())
            .await;
        if database.is_err() {
            panic!("Database connection disappeared");
        }
        let database = database.unwrap().database("inventory");
        //create index
        {
            let idx = IndexModel::builder()
                .keys(doc! { "content": "text","title":"text" })
                .options(
                    IndexOptions::builder()
                        .name("threads-index".to_string())
                        .unique(false)
                        .build()
                )
                .build();
            let _index = &database
                .collection::<Thread>("threads")
                .create_index(idx).await;
        }
        let redis = Client::open(env.redis_url.clone());
        if redis.is_err() {
            panic!("url {} -> {}", env.redis_url.clone(), redis.unwrap_err())
        }
        let redis = redis.unwrap();

        let redis_util = RedisClient::from(redis, env.mode.clone());

        AppState {
            sse,
            db: database,
            redis: redis_util,
        }
    }
}
