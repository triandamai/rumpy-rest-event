use axum::response::IntoResponse;

use crate::common::api_response::ApiResponse;

pub async fn index() -> Result<impl IntoResponse, String> {
    
    Ok(ApiResponse::ok("sas".to_string(), "sasa"))
}

