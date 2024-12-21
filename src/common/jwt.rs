use std::collections::HashMap;
use std::ops::Add;
use std::string::ToString;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use bson::oid::ObjectId;
use chrono::{Duration, Local};
use jsonwebtoken::{
    errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use super::app_state::AppState;
use crate::common::api_response::ApiResponse;
use crate::common::env_config::EnvConfig;
use crate::common::permission::permission::app;
use crate::common::utils::{create_object_id_option, create_or_new_object_id};
use crate::feature::auth::auth_model::{BRANCH_ID_KEY, USER_ID_KEY};
use crate::translate;

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
    pub permissions: HashMap<String, String>,
    pub branch_id: Option<ObjectId>,
    pub user_id: Option<ObjectId>,
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
    pub fn is_branch_id_equal(&self, id: &ObjectId) -> bool {
        if self.branch_id.is_none() {
            return false;
        }
        let branch_id = self.branch_id.unwrap();
        branch_id.eq(id)
    }
    pub fn authorize(&self, permission: &str) -> bool {
        if self.permissions.contains_key(app::admin::ALL) {
            return true;
        }
        self.permissions.get(&permission.to_string()).is_some()
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

    pub fn authorize_result(&self, permission: &str) -> Result<bool,ApiResponse<String>> {
        if self.permissions.contains_key(app::admin::ALL) {
            return Ok(true);
        }
        let allow = self.permissions.get(&permission.to_string()).is_some();

        if allow {
            return Ok(allow);
        }

        Err(ApiResponse::un_authorized(
            translate!("unauthorized").as_str(),
        ))
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
        Err(ApiResponse::un_authorized(
            translate!("unauthorized").as_str(),
        ))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token =
            JwtUtil::decode(bearer.token().to_string()).map_err(|_| AuthError::InvalidToken);

        match token {
            Ok(token) => Ok(token.claims),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthContext
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, s: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token =
            JwtUtil::decode(bearer.token().to_string()).map_err(|_| AuthError::InvalidToken);

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
        let default = String::from("");
        let session = state.redis.get_session_sign_in(claims.sub.clone().as_str());

        if session.is_err() {
            return Err(AuthError::MissingCredentials);
        }

        let session = &session.unwrap();

        if session.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        let user_id = session
            .get(USER_ID_KEY)
            .map_or(None, |s| create_object_id_option(s.as_str()));
        let branch_id = session.get(BRANCH_ID_KEY).unwrap_or(&default);
        let branch_id = create_or_new_object_id(branch_id);

        let permissions = &state
            .redis
            .get_session_permission(claims.sub.clone().as_str())
            .map_or(default_hashmap.clone(), |p| p);

        Ok(AuthContext {
            claims,
            branch_id,
            user_id,
            permissions: permissions.clone(),
        })
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
