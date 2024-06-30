use axum::body::HttpBody;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status_code: i32,
    pub data: Option<T>,
    pub message: &'static str,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T, message: &'static str) -> ApiResponse <T> {
        ApiResponse {
            status_code: 200,
            data: Some(data),
            message,
        }
    }
    pub fn created(data: T, message: &'static str) -> ApiResponse <T> {
        ApiResponse {
            status_code: 201,
            data: Some(data),
            message,
        }
    }

    pub fn access_denied(message: &'static str) -> ApiResponse <T> {
        ApiResponse {
            status_code: 503,
            data: None,
            message,
        }
    }
    pub fn un_authorized(message: &'static str) -> ApiResponse <T> {
        ApiResponse {
            status_code: 401,
            data: None,
            message,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T> where T: Serialize {
    fn into_response(self) -> Response {
        if self.status_code == 401 {
            return (StatusCode::UNAUTHORIZED, Json(self)).into_response();
        }
        if self.status_code == 503 {
            return (StatusCode::FORBIDDEN, Json(self)).into_response();
        }
        Json(self).into_response()
    }
}