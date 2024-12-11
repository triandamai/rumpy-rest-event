use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bson::doc;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagingResponse<T> {
    pub total_items: i64,
    pub total_pages: i64,
    pub page: i64,
    pub size: i64,
    pub items: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub q: Option<String>,
    pub filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    code: i32,
    message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldError {
    name: String,
    error_message: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub meta: Meta,
    pub errors: Option<Vec<FieldError>>,
}

impl<T> ApiResponse<T> {
    pub fn create(status_code: i32, data: T, message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: status_code,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn ok(data: T, message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: 200,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn created(data: T, message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: Some(data),
            meta: Meta {
                code: 201,
                message: message.to_string(),
            },
            errors: None,
        }
    }

    pub fn not_found(message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 404,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn bad_request(message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 400,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn error_validation(error: ValidationErrors, message: &str) -> ApiResponse<T> {
        let field = error
            .field_errors()
            .iter()
            .map(|(key, value)| {
                let message: Vec<String> = value
                    .iter()
                    .map(|e| {
                        if e.message.is_none() {
                            return e.code.clone().to_string();
                        }
                        return match e.message.clone() {
                            None => e.code.to_string(),
                            Some(v) => v.to_string(),
                        };
                    })
                    .collect::<Vec<String>>();
                return FieldError {
                    name: key.to_string(),
                    error_message: message,
                };
            })
            .collect::<Vec<FieldError>>();

        ApiResponse {
            data: None,
            meta: Meta {
                code: 400,
                message: message.to_string(),
            },
            errors: Some(field),
        }
    }
    pub fn failed(message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 400,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn access_denied(message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 403,
                message: message.to_string(),
            },
            errors: None,
        }
    }
    pub fn un_authorized(message: &str) -> ApiResponse<T> {
        ApiResponse {
            data: None,
            meta: Meta {
                code: 401,
                message: message.to_string(),
            },
            errors: None,
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
