use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::{Rng, rng};
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
    pub fn create_key_otp_session(&self, session_id: &str) -> String {
        format!("{}:session:otp:{}", self.mode, session_id)
    }
    pub fn create_key_permission_session(&self, session_id: &str) -> String {
        format!("{}:session:permission:{}", self.mode, session_id)
    }
    pub fn create_key_sign_in_session(&self, session_id: &str) -> String {
        format!("{}:session:sign_in:{}", self.mode, session_id)
    }
    pub fn create_key_sign_up_session(&self, session_id: &str) -> String {
        format!("{}:session:sign_up:{}", self.mode, session_id)
    }
    pub fn create_key_reset_password_session(&self, session_id: &str) -> String {
        format!("{}:session:reset_password:{}", self.mode, session_id)
    }

    //SESSION OTP
    pub fn set_session_otp(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_otp_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, 3600);
        saved
    }

    pub fn get_session_otp(&mut self, session_id: &str) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_otp_session(session_id);
        self.client.hgetall(key)
    }
    //END OTP

    //SET PERMISSION
    pub fn set_session_permission(
        &mut self,
        session_id: &str,
        items: Vec<(String, String)>,
    ) -> RedisResult<String> {
        let key = self.create_key_permission_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), &*items);
        let _: RedisResult<String> = self.client.expire(key, 3600);
        saved
    }

    pub fn get_session_permission(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_permission_session(session_id);
        self.client.hgetall(key)
    }

    pub fn delete_session_permission(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_permission_session(session_id);
        self.client.del(key)
    }
    //END PERMISSION

    //SET SESSION SIGN UP
    pub fn set_session_sign_up(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_sign_up_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, 3600);
        saved
    }

    //GET SESSION SIGN UP
    pub fn get_session_sign_up(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_sign_up_session(session_id);
        self.client.hgetall(key)
    }

    //SET SESSION
    pub fn set_session_sign_in(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_sign_in_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, 3600);
        saved
    }

    //GET SESSION
    pub fn get_session_sign_in(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_sign_in_session(session_id);
        self.client.hgetall(key)
    }

    //SET SESSION SIGN UP
    pub fn set_session_reset_password(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_reset_password_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, 3600);
        saved
    }

    //GET SESSION SIGN UP
    pub fn get_session_reset_password(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_reset_password_session(session_id);
        self.client.hgetall(key)
    }
    pub fn delete_session_otp(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_otp_session(session_id);
        self.client.del(key)
    }
    //DELETE SESSION
    pub fn delete_session_sign_up(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_sign_up_session(session_id);
        self.client.del(key)
    }
    //DELETE SESSION
    pub fn delete_session_sign_in(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_sign_in_session(session_id);
        self.client.del(key)
    }

    //DELETE SESSION
    pub fn delete_session_reset_password(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_reset_password_session(session_id);
        self.client.del(key)
    }

    //OTHER
    pub async fn exist(&mut self, key: &str) -> bool {
        let result: RedisResult<i16> = self.client.exists(key);
        if result.is_err() {
            return false;
        }
        if result.unwrap() < 1 {
            return false;
        }
        true
    }

    pub fn generate_otp(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30;
        let mut rng = rng();
        let random_number: u32 = rng.random_range(0..10000);
        let otp = (now as u32 ^ random_number) % 10000;
        if otp.to_string().len() < 4 {
            return format!("{}{}", otp, rng.random_range(0..10));
        }
        format!("{}", otp)
    }

    pub fn get_list_subscriber(&mut self, topic: String) -> HashMap<String, String> {
        let subscriber: RedisResult<HashMap<String, String>> =
            self.client.hgetall(format!("sse:{}", topic));

        subscriber.map_or_else(|_| HashMap::new(), |v| v)
    }

    pub fn subscribe_to_topic(&mut self, topic: String, user_id: String) -> Result<String, String> {
        let subscribe: RedisResult<String> =
            self.client.hset(topic, user_id.clone(), user_id.clone());
        subscribe.map_or_else(|_| Err("".to_string()), |value| Ok(value))
    }

    pub fn remove_to_topic(&mut self, topic: String, user_id: String) -> Result<String, String> {
        let subscribe: RedisResult<String> = self.client.hdel(topic, user_id.clone());
        subscribe.map_or_else(|_| Err("".to_string()), |value| Ok(value))
    }
}
