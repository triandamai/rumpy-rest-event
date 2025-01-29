use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use file_model::QueryGetFile;

use crate::{
    common::{api_response::ApiResponse, app_state::AppState, lang::Lang, minio::MinIO},
    translate,
};
pub mod file_model;

pub async fn get_file(
    _state: State<AppState>,
    lang: Lang,
    Path(bucket): Path<String>,
    query: Query<QueryGetFile>,
) -> impl IntoResponse {
    if query.file_name.is_none() {
        return ApiResponse::<String>::bad_request(
            translate!("file.get.file_name.null", lang).as_str(),
        )
        .into_response();
    }

    let minio = MinIO::new().await;

    let default = String::new();
    let get = minio
        .get_file(bucket, query.file_name.clone().unwrap_or(default))
        .await;

    if get.is_err() {
        return ApiResponse::<String>::not_found(translate!("file.get.not-found", lang).as_str())
            .into_response();
    }
    let file = get.unwrap();

    axum::body::Body::from(file.into_bytes()).into_response()
}
