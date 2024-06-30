use redis::{Client, Commands, RedisResult};

#[derive(Clone,Debug)]
pub struct RedisUtil {
    client: Client,
    mode: String,
}

impl RedisUtil {
    pub fn from(redis_client: Client, mode: String) -> Self {
        RedisUtil {
            client: redis_client,
            mode,
        }
    }

    pub fn generate_user_session_key(&self, session_id: &str) -> String {
        format!("{}:session:{}", self.mode, session_id)
    }

    pub async fn exist(&mut self, key: &str) -> bool {
        let result: RedisResult<i16> = self.client.exists(key);
        if result.is_err() {
            return false;
        }
        if result.unwrap() < 1 {
            return false;
        }
        return true;
    }

}