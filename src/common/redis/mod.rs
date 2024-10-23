use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::{thread_rng, Rng};
use redis::{Client, Commands, RedisResult};

use crate::feature::auth::auth_model::{
    ATTEMPT_KEY, OTP_KEY, OTP_TTL, RESEND_ATTEMPT_KEY, SIGN_IN_TTL, SIGN_UP_TTL,
};

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
        format!("{}:session:sign_in:{}", self.mode, session_id)
    }

    pub fn create_key_sign_up_session(&self, session_id: &str) -> String {
        format!("{}:session:sign_up:{}", self.mode, session_id)
    }

    pub fn create_key_forgot_password_session(&self, session_id: &str) -> String {
        format!("{}:session:forgot_password:{}", self.mode, session_id)
    }

    pub fn create_key_otp_sign_in_session(&self, session_id: &str) -> String {
        format!("{}:otp:sign_in:{}", self.mode, session_id)
    }

    pub fn create_key_otp_sign_up_session(&self, session_id: &str) -> String {
        format!("{}:otp:sign_up:{}", self.mode, session_id)
    }

    pub fn create_key_otp_forgot_password_session(&self, session_id: &str) -> String {
        format!("{}:otp:forgot_password:{}", self.mode, session_id)
    }

    //SET SESSION
    pub fn set_session_sign_in(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_sign_in_session(session_id);
        let saved: RedisResult<String> = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, SIGN_IN_TTL);
        saved
    }

    pub fn set_session_sign_up(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_sign_up_session(session_id);
        let saved = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, SIGN_UP_TTL);
        saved
    }

    pub fn set_session_forgot_password(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_forgot_password_session(session_id);
        let saved = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, SIGN_UP_TTL);
        saved
    }

    //SET OTP
    pub fn set_otp_session_sign_in(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_in_session(session_id);
        let saved = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    pub fn set_otp_session_sign_up(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_up_session(session_id);
        let saved = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    pub fn set_otp_session_forgot_password(
        &mut self,
        session_id: &str,
        items: &[(&str, String)],
    ) -> RedisResult<String> {
        let key = self.create_key_otp_forgot_password_session(session_id);
        let saved = self.client.hset_multiple(key.clone(), items);
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    //CHANGE OTP
    pub fn change_otp_session_sign_in(
        &mut self,
        session_id: &str,
        new_otp: String,
        resend_attmpt: i64,
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_in_session(session_id);
        let saved = self.client.hset_multiple(
            key.clone(),
            &[
                (OTP_KEY, new_otp),
                (ATTEMPT_KEY, "0".to_string()),
                (RESEND_ATTEMPT_KEY, resend_attmpt.to_string()),
            ],
        );
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    pub fn change_otp_session_sign_up(
        &mut self,
        session_id: &str,
        new_otp: String,
        resend_attempt: i32,
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_up_session(session_id);
        let saved = self.client.hset_multiple(
            key.clone(),
            &[
                (OTP_KEY, new_otp),
                (ATTEMPT_KEY, "0".to_string()),
                (RESEND_ATTEMPT_KEY, resend_attempt.to_string()),
            ],
        );
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    pub fn change_otp_session_forgot_password(
        &mut self,
        session_id: &str,
        new_otp: String,
        resend_attmpt: i64,
    ) -> RedisResult<i64> {
        let key = self.create_key_otp_forgot_password_session(session_id);
        let saved = self.client.hset_multiple(
            key.clone(),
            &[
                (OTP_KEY, new_otp),
                (ATTEMPT_KEY, "0".to_string()),
                (RESEND_ATTEMPT_KEY, resend_attmpt.to_string()),
            ],
        );
        let _: RedisResult<String> = self.client.expire(key, OTP_TTL);
        saved
    }

    //SET ATTEMMPT
    pub fn set_otp_attempt_sign_in(
        &mut self,
        session_id: &str,
        attempt: i32,
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_in_session(session_id);
        self.client
            .hset_multiple(key, &[(ATTEMPT_KEY, attempt.to_string())])
    }

    pub fn set_otp_attempt_sign_up(
        &mut self,
        session_id: &str,
        attempt: i32,
    ) -> RedisResult<String> {
        let key = self.create_key_otp_sign_up_session(session_id);
        self.client
            .hset_multiple(key, &[(ATTEMPT_KEY, attempt.to_string())])
    }

    pub fn set_otp_attempt_forgot_password(
        &mut self,
        session_id: &str,
        attempt: i32,
    ) -> RedisResult<String> {
        let key = self.create_key_otp_forgot_password_session(session_id);
        self.client
            .hset_multiple(key, &[(ATTEMPT_KEY, attempt.to_string())])
    }

    //GET SESSION
    pub fn get_session_sign_in(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_sign_in_session(session_id);
        self.client.hgetall(key)
    }
    pub fn get_session_sign_up(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_sign_up_session(session_id);
        self.client.hgetall(key)
    }
    pub fn get_session_forgot_password(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_forgot_password_session(session_id);
        self.client.hgetall(key)
    }

    //GET SESSION OTP
    pub fn get_session_otp_sign_in(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_otp_sign_in_session(session_id);
        self.client.hgetall(key)
    }
    pub fn get_session_otp_sign_up(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_otp_sign_up_session(session_id);
        self.client.hgetall(key)
    }
    pub fn get_session_otp_forgot_password(
        &mut self,
        session_id: &str,
    ) -> RedisResult<HashMap<String, String>> {
        let key = self.create_key_otp_forgot_password_session(session_id);
        self.client.hgetall(key)
    }

    //DELETE OTP
    pub fn delete_otp_session_sign_in(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_otp_sign_in_session(session_id);
        self.client.del(key)
    }

    pub fn delete_otp_session_sign_up(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_otp_sign_up_session(session_id);
        self.client.del(key)
    }

    pub fn delete_otp_session_forgot_password(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_otp_forgot_password_session(session_id);
        self.client.del(key)
    }

    //DELETE SESSION
    pub fn delete_session_sign_in(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_sign_in_session(session_id);
        self.client.del(key)
    }

    pub fn delete_session_sign_up(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_sign_up_session(session_id);
        self.client.del(key)
    }

    pub fn delete_session_forgot_password(&mut self, session_id: &str) -> RedisResult<String> {
        let key = self.create_key_forgot_password_session(session_id);
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

    pub fn get_list_subscriber(&mut self,topic:String)->HashMap<String, String>{
        let subscriber:RedisResult<HashMap<String, String>> = self.client
            .hgetall(format!("sse:{}", topic));

        subscriber.map_or_else(|_|HashMap::new(),|v|v)
    }

    pub fn subscribe_to_topic(&mut self,topic:String,user_id:String)->Result<String,String>{
        let subscribe:RedisResult<String> = self
            .client
            .hset(topic,user_id.clone(),user_id.clone());
       subscribe.map_or_else(|_|Err("".to_string()),|value|Ok(value))
    }

    pub fn remove_to_topic(&mut self,topic:String,user_id:String)->Result<String,String>{
        let subscribe:RedisResult<String> = self
            .client
            .hdel(topic,user_id.clone());
        subscribe.map_or_else(|_|Err("".to_string()),|value|Ok(value))
    }
}
