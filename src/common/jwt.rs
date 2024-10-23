use std::ops::Add;
use std::string::ToString;

use axum::{async_trait, RequestPartsExt};
use axum::extract::{FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::{
    headers::{Authorization, authorization::Bearer},
    TypedHeader,
};
use chrono::{Duration, Local};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, errors::Error as JwtError, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::common::api_response::ApiResponse;
use crate::common::env_config::EnvConfig;

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
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}


const JWT_SECRET_KEY: &str = "JWT_SECRET";
const JWT_SECRET_KEY_DEFAULT: &str = "triandamai";
const ISS: &str = "bluhabit.id";

impl JwtUtil {
    pub fn encode(
        sub: String
    ) -> Option<String> {
        let secret = EnvConfig::init();
        let exp = Local::now().add(Duration::hours(3)).timestamp();
        let claims = JwtClaims {
            iss: ISS.to_string(),
            sub,
            iat: Local::now().timestamp(),
            exp,
        };

        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.jwt_secret.as_ref()),
        ).unwrap();
        Some(token)
    }

    pub fn decode(
        token: String
    ) -> Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> {
        let secret =EnvConfig::init();
        let decoded: Result<TokenData<JwtClaims>, JwtError> = jsonwebtoken::decode::<JwtClaims>(
            &token,
            &DecodingKey::from_secret(secret.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        decoded
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

        let token = JwtUtil::decode(bearer.token().to_string())
            .map_err(|_| AuthError::InvalidToken);

        if token.is_err() {
            return Err(token.unwrap_err());
        }
        Ok(token.unwrap().claims)
    }
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let response = match self {
            AuthError::WrongCredentials => ApiResponse::<String>::un_authorized("Wrong credentials"),
            AuthError::MissingCredentials => ApiResponse::<String>::un_authorized("Missing credentials"),
            AuthError::TokenCreation => ApiResponse::<String>::un_authorized("Token creation error"),
            AuthError::InvalidToken => ApiResponse::<String>::un_authorized("Invalid token")
        };

        response.into_response()
    }
}
