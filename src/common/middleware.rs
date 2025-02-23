use crate::common::api_response::ApiResponse;
use crate::common::multipart_file::{MultiFileExtractor, SingleFileExtractor};
use axum::extract::multipart::MultipartRejection;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Multipart, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json as AxumJson;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Debug)]
pub struct Error {
    pub response: String,
    pub text: String,
}

impl Error {
    pub fn new(text: &str) -> Self {
        Self {
            response: "ERROR".to_string(),
            text: text.to_string(),
        }
    }
}

//https://github.com/tokio-rs/axum/discussions/932
pub async fn method_not_allowed(req: Request, next: Next) -> impl IntoResponse {
    let resp = next.run(req).await;
    let status = resp.status();

    match status {
        StatusCode::METHOD_NOT_ALLOWED => Err((
            StatusCode::METHOD_NOT_ALLOWED,
            AxumJson(ApiResponse::create(
                405,
                None::<String>,
                "Method Not Allowed",
            )),
        )
            .into_response()),
        _ => Ok(resp),
    }
}

//https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/custom_extractor.rs
pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts.clone(), body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = json!({
                    "meta": {
                        "code":422,
                        "message":format!("{}",rejection.body_text())
                    },
                    "data":None::<String>,
                    "error": None::<String>,
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}

//https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/custom_extractor.rs

impl<S> FromRequest<S> for SingleFileExtractor
where
    Multipart: FromRequest<S, Rejection = MultipartRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts.clone(), body);

        match Multipart::from_request(req, state).await {
            Ok(value) => {
                let file = SingleFileExtractor::extract(value).await;
                if file.is_error {
                    let default = String::new();
                    let msg = file.error_message.unwrap_or(default);
                    let payload = json!({
                        "meta": {
                            "code":422,
                            "message":format!("{}",msg)
                        },
                        "data":None::<String>,
                        "error": None::<String>,
                    });
                    return Err((StatusCode::BAD_REQUEST, axum::Json(payload)));
                }
                Ok(file)
            }
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = json!({
                    "meta": {
                        "code":422,
                        "message":format!("{}",rejection.body_text())
                    },
                    "data":None::<String>,
                    "error": None::<String>,
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}

impl<S> FromRequest<S> for MultiFileExtractor
where
    Multipart: FromRequest<S, Rejection = MultipartRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts.clone(), body);

        match Multipart::from_request(req, state).await {
            Ok(value) => {
                let file = MultiFileExtractor::extract(value).await;
                if file.is_error {
                    let default = String::new();
                    let msg = file.error_message.unwrap_or(default);
                    let payload = json!({
                        "meta": {
                            "code":422,
                            "message":format!("{}",msg)
                        },
                        "data":None::<String>,
                        "error": None::<String>,
                    });
                    return Err((StatusCode::BAD_REQUEST, axum::Json(payload)));
                }
                Ok(file)
            }
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = json!({
                    "meta": {
                        "code":422,
                        "message":format!("{}",rejection.body_text())
                    },
                    "data":None::<String>,
                    "error": None::<String>,
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}
