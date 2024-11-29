use crate::common::jwt::AuthError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait};

#[derive(Debug)]
pub struct Lang {
    pub locale_code: String,
}

impl Lang {
    pub fn get(&self) -> &str {
        self.locale_code.as_str()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Lang
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let head = parts.headers.get("Accept-Language");
        if head.is_none() {
            return Ok(Lang {
                locale_code: "id-ID".to_string(),
            });
        }
        let head = head.unwrap();

        Ok(Lang {
            locale_code: head.to_str().unwrap_or("id-ID").to_string(),
        })
    }
}
