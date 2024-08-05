use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, thread_rng};
use redis::{Client, Commands, RedisResult};

#[derive(Clone, Debug)]
pub struct RedisClient {
    pub client: Client,
    mode: String,
}

impl RedisClient {
    pub fn from(redis_client: Client, mode: String) -> Self {
        RedisClient {
            client: redis_client,
            mode,
        }
    }
    pub fn create_key_sign_in_session(&self, session_id: &str) -> String {
        format!("{}:session:{}", self.mode, session_id)
    }

    pub fn create_key_otp_session(&self, session_id: &str) -> String {
        format!("{}:otp:{}", self.mode, session_id)
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

    pub fn generate_otp(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30;
        let mut rng = thread_rng();
        let random_number: u32 = rng.gen_range(0..10000);
        let otp = (now as u32 ^ random_number) % 10000;
        if otp.to_string().len() < 4 {
            return format!("{}{}", otp, rng.gen_range(0..10));
        }
        format!("{}", otp)
    }
}
