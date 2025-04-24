use google_oauth::{AsyncClient, GoogleAccessTokenPayload, GooglePayload};
use log::info;

use super::env_config::EnvConfig;

pub async fn validate_google_id_token(token: &str) -> Result<GooglePayload, String> {
    let env = EnvConfig::init();

    let client = AsyncClient::new(&env.google_client_id);

    let payload = client.validate_id_token(token).await;
    if let Err(why) = payload {
        info!(target:"validate-google-id-token","{:?}",why);
        return Err(why.to_string());
    }

    let payload = payload.unwrap();
    Ok(payload)
}

pub async fn validate_google_oauth(token: &str) -> Result<GoogleAccessTokenPayload, String> {
    let client = AsyncClient::new("");

    let payload = client.validate_access_token(token).await;
    if let Err(why) = payload {
        info!(target:"validate-google-access-token","{:?}",why);
        return Err(why.to_string());
    }

    let payload = payload.unwrap();
    Ok(payload)
}
