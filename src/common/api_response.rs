use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Type,FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
#[sqlx(type_name = "count")]
pub struct Count {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PagingResponse<T> {
    pub total_items: i32,
    pub total_pages: i32,
    pub items: Vec<T>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i32>,
    pub size: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    code: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub meta: Meta,
}

impl<T> ApiResponse<T> {
    pub fn create(status_code: i32, data: T, message: &'static str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: status_code,
                message: message.to_string(),
            },
        }
    }
    pub fn ok(data: T, message: &'static str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: 200,
                message: message.to_string(),
            },
        }
    }
    pub fn created(data: T, message: &'static str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: 201,
                message: message.to_string(),
            },
        }
    }
    pub fn failed(message: String) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 400,
                message: message,
            },
        }
    }
    pub fn access_denied(message: &'static str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 403,
                message: message.to_string(),
            },
        }
    }
    pub fn un_authorized(message: &'static str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 401,
                message: message.to_string(),
            },
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        //follow http
        if self.meta.code >= 100 && self.meta.code <= 599 {
            return (
                StatusCode::from_u16(self.meta.code as u16).unwrap(),
                Json(self),
            )
                .into_response();
        }
        //custom error
        if self.meta.code >= 600 {
            return (StatusCode::BAD_REQUEST, Json(self)).into_response();
        }

        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}
