use std::collections::HashMap;
use std::ops::Add;
use std::string::ToString;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::RequestPartsExt;
use bson::oid::ObjectId;
use chrono::{Duration, Local};
use jsonwebtoken::{
    errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use log::info;
use serde::{Deserialize, Serialize};

use super::app_state::AppState;
use crate::common::api_response::ApiResponse;
use crate::common::constant::REDIS_KEY_USER_ID;
use crate::common::env_config::EnvConfig;
use crate::common::permission::permission::app;
use crate::common::utils::create_object_id_option;

pub struct JwtUtil {
    pub claims: JwtClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthContext {
    pub claims: JwtClaims,
    pub session: HashMap<String, String>,
    pub permissions: HashMap<String, String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

const ISS: &str = "strong-teams.id";

impl JwtUtil {
    pub fn encode(sub: String) -> Option<String> {
        info!(target:"app::Jwt","encode");
        let secret = EnvConfig::init();
        let exp = Local::now().add(Duration::hours(12)).timestamp();
        let claims = JwtClaims {
            iss: ISS.to_string(),
            sub,
            iat: Local::now().timestamp(),
            exp,
        };

        match jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.jwt_secret.as_ref()),
        ) {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }

    pub fn decode(token: String) -> Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> {
        info!(target:"app::Jwt","decode");
        let secret = EnvConfig::init();
        let decoded: Result<TokenData<JwtClaims>, JwtError> = jsonwebtoken::decode::<JwtClaims>(
            &token,
            &DecodingKey::from_secret(secret.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        decoded
    }
}

impl AuthContext {
    pub fn authorize(&self, permission: &str) -> bool {
        if self.permissions.contains_key(app::admin::ALL) {
            return true;
        }
        self.permissions.get(&permission.to_string()).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.session.get(key)
    }

    pub fn get_user_id(&self) -> Option<ObjectId> {
        self.get(REDIS_KEY_USER_ID).map_or_else(|| None, |id| create_object_id_option(id))
    }

    pub fn authorize_multiple(&self, permissions: Vec<&str>) -> bool {
        if self.permissions.contains_key(app::admin::ALL) {
            return true;
        }
        permissions
            .iter()
            .map(|v| v.to_string())
            .all(|v| self.permissions.get(&v).is_some())
    }

    pub fn authorize_result(&self, permission: &str) -> Result<bool, ApiResponse<String>> {
        if self.permissions.contains_key(app::admin::ALL) {
            return Ok(true);
        }
        let allow = self.permissions.get(&permission.to_string()).is_some();

        if allow {
            return Ok(allow);
        }

        Err(ApiResponse::un_authorized("Unauthorized"))
    }

    pub fn authorize_multiple_result(
        &self,
        permissions: Vec<&str>,
    ) -> Result<bool, ApiResponse<String>> {
        if self.permissions.contains_key(app::admin::ALL) {
            return Ok(true);
        }
        let allow = permissions
            .iter()
            .map(|v| v.to_string())
            .all(|v| self.permissions.get(&v).is_some());
        if allow {
            return Ok(true);
        }
        Err(ApiResponse::un_authorized("Unauthorized"))
    }
}

impl<S> FromRequestParts<S> for AuthContext
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, s: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.headers.get("authorization") {
            if let Ok(auth_str) = auth.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let token =
                        JwtUtil::decode(token.to_string()).map_err(|_| AuthError::InvalidToken);

                    if token.is_err() {
                        return Err(token.unwrap_err());
                    }
                    let sub = token.unwrap();
                    let claims = sub.claims;
                    let state: Result<AppState, AuthError> = parts
                        .extract_with_state::<AppState, _>(s)
                        .await
                        .map_err(|_| AuthError::MissingCredentials);

                    if state.is_err() {
                        return Err(AuthError::MissingCredentials);
                    }
                    let mut state = state.unwrap();

                    let default_hashmap: HashMap<String, String> = HashMap::new();

                    let session = state.redis.get_session_sign_in(claims.sub.clone().as_str());

                    if session.is_err() {
                        return Err(AuthError::MissingCredentials);
                    }

                    let session = &session.unwrap();

                    if session.is_empty() {
                        return Err(AuthError::MissingCredentials);
                    }

                    let permissions = &state
                        .redis
                        .get_session_permission(claims.sub.clone().as_str())
                        .map_or(default_hashmap.clone(), |p| p);

                    Ok(AuthContext {
                        claims,
                        permissions: permissions.clone(),
                        session: session.clone(),
                    })
                } else {
                    return Err(AuthError::MissingCredentials);
                }
            } else {
                return Err(AuthError::MissingCredentials);
            }
        } else {
            return Err(AuthError::MissingCredentials);
        }
    }
}


impl<S> FromRequestParts<S> for JwtClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _s: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.headers.get("authorization") {
            if let Ok(auth_str) = auth.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let token =
                        JwtUtil::decode(token.to_string()).map_err(|_| AuthError::InvalidToken);

                    if token.is_err() {
                        return Err(token.unwrap_err());
                    }
                    let sub = token?;

                    let claims = sub.claims;

                    Ok(claims)
                } else {
                    Err(AuthError::MissingCredentials)
                }
            } else {
                Err(AuthError::MissingCredentials)
            }
        } else {
            Err(AuthError::MissingCredentials)
        }
    }
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let response = match self {
            AuthError::WrongCredentials => {
                ApiResponse::<String>::un_authorized("Wrong credentials")
            }
            AuthError::MissingCredentials => {
                ApiResponse::<String>::un_authorized("Missing credentials")
            }
            AuthError::TokenCreation => {
                ApiResponse::<String>::un_authorized("Token creation error")
            }
            AuthError::InvalidToken => ApiResponse::<String>::un_authorized("Invalid token"),
        };

        response.into_response()
    }
}
