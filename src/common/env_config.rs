use dotenv::dotenv;
use log::info;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub database_url: String,
    pub base_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub minio_url: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub mode: String,
}

impl EnvConfig {
    pub fn init() -> Arc<EnvConfig> {
        info!(target: "env::config","Initializing env config...");
        dotenv().ok();
        let env_mode = std::env::var("MODE").unwrap_or("DEV".to_string());

        let database_env_key = format!("DATABASE_URL_{}", env_mode);
        let redis_env_key = format!("REDIS_URL_{}", env_mode);
        let minio_env_key = format!("MINIO_URL_{}", env_mode);
        let minio_env_access_key = format!("MINIO_ACCESS_KEY_{}", env_mode);
        let minio_env_secret_key = format!("MINIO_SECRET_KEY_{}", env_mode);
        let jwt_env_key = format!("JWT_SECRET_{}", env_mode);
        let base_url_key = format!("BASE_URL_{}", env_mode);

        let env_base_url = std::env::var(base_url_key.clone());
        let env_database = std::env::var(database_env_key.clone());
        let env_redis = std::env::var(redis_env_key.clone());
        let env_minio = std::env::var(minio_env_key.clone());
        let env_minio_access_key = std::env::var(minio_env_access_key.clone());
        let env_minio_secret_key = std::env::var(minio_env_secret_key.clone());
        let env_jwt = std::env::var(jwt_env_key.clone());

        if env_base_url.is_err() {
            panic!(
                "Cannot load env base_url {} mode, error={}",
                database_env_key,
                env_base_url.unwrap_err().to_string()
            )
        }
        if env_database.is_err() {
            panic!(
                "Cannot load env database {} mode, error={}",
                database_env_key,
                env_database.unwrap_err().to_string()
            )
        }

        if env_redis.is_err() {
            panic!(
                "Cannot load env redis {} mode, error={}",
                redis_env_key,
                env_redis.unwrap_err().to_string()
            )
        }

        if env_minio.is_err() {
            panic!(
                "Cannot load env minio {} mode, error={}",
                redis_env_key,
                env_redis.unwrap_err().to_string()
            )
        }
        if env_minio_access_key.is_err() {
            panic!(
                "Cannot load env minio access key {} mode, error={}",
                redis_env_key,
                env_redis.unwrap_err().to_string()
            )
        }
        if env_minio_secret_key.is_err() {
            panic!(
                "Cannot load env minio secret key {} mode, error={}",
                redis_env_key,
                env_redis.unwrap_err().to_string()
            )
        }
        if env_jwt.is_err() {
            panic!(
                "Cannot load env jwt {} mode, error={}",
                redis_env_key,
                env_redis.unwrap_err().to_string()
            )
        }
        info!(target: "env::config","Finish Init env config...");
        Arc::new(EnvConfig {
            database_url: env_database.unwrap(),
            base_url: env_base_url.unwrap(),
            redis_url: env_redis.unwrap(),
            minio_url: env_minio.unwrap(),
            minio_access_key: env_minio_access_key.unwrap(),
            minio_secret_key: env_minio_secret_key.unwrap(),
            jwt_secret: env_jwt.unwrap(),
            mode: env_mode,
        })
    }
}
