use std::collections::HashMap;
use log::info;
use super::env_config::EnvConfig;

pub async fn send_otp(phone: String, otp: String) -> Result<String, String> {
    let is_test = vec!["6281226809435", "628111111111"]
        .into_iter()
        .any(|v| v.to_string() == phone.clone());
    if !is_test {
        let env = EnvConfig::init();
        let mut body_request: HashMap<&str, String> = HashMap::new();
        body_request.insert("target", phone);
        body_request.insert("message", format!("Your otp is *{}*", otp));
        let send_otp = reqwest::Client::new();
        let send_otp = send_otp
            .post(env.wa_url.clone())
            .json(&body_request)
            .header("Authorization", env.wa_token.clone())
            .send()
            .await;

        match send_otp {
            Ok(res) => {
                if res.status().is_success() {
                    Ok("".to_string())
                } else {
                    Err(format!("error with {:?} res: {:?}", res.status(), res))
                }
            }
            Err(why) => Err(format!("{:?}", why)),
        }
    }else {
        info!(target:"wa::api","Skip during test number  otp:{}", otp);
        Ok(format!("Skip during test number  otp:{}", otp))
    }
}
