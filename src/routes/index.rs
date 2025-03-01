use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;

use axum::extract::State;
use std::fs::File;
use std::io::Write;

pub async fn index(_state: State<AppState>, _lang: Lang) -> ApiResponse<String> {
    return ApiResponse::ok("OK".to_string(), "");
}

pub async fn generate_locales() -> ApiResponse<String> {
    let minio = MinIO::new()
        .get_file("assets".to_string(), "locales.json".to_string())
        .await;
    if minio.is_err() {
        return ApiResponse::failed(minio.unwrap_err().as_str());
    }
    let file = minio.unwrap();

    // Read the file contents
    let data = file.bytes();

    // Save the file to the "uploads" folder
    let mut file = File::create(format!("locales/{}", "app.json")).unwrap();
    file.write_all(&data).unwrap();
    ApiResponse::ok(String::from("OK"), "success generate locales")
}
