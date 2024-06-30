use std::sync::Arc;
use dotenv::dotenv;

pub struct EnvConfig {
    pub postgres_url: String,
    pub redis_url: String,
    pub mode: String,
}

impl EnvConfig {
    pub fn init() -> Arc<EnvConfig> {
        dotenv().ok();
        let env_mode = std::env::var("MODE")
            .unwrap_or("DEV".to_string());

        let postgres_env_key = format!("POSTGRES_URL_{}", env_mode);
        let redis_env_key = format!("REDIS_URL_{}", env_mode);

        let env_postgres = std::env::var(postgres_env_key.clone());
        let env_redis = std::env::var(redis_env_key.clone());

        if env_postgres.is_err() {
            panic!("Cannot load env postgres {} mode, error={}", postgres_env_key, env_postgres.unwrap_err().to_string())
        }

        if env_redis.is_err() {
            panic!("Cannot load env redis {} mode, error={}", redis_env_key, env_redis.unwrap_err().to_string())
        }

        Arc::new(EnvConfig {
            postgres_url: env_postgres.unwrap(),
            redis_url: env_redis.unwrap(),
            mode: env_mode,
        })
    }
}