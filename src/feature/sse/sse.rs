use std::convert::Infallible;

use axum::extract::{Query, State};
use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use axum::Json;
use futures::Stream;

use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;
use crate::common::sse::sse_builder::{SseBuilder, SseTarget};
use crate::feature::sse::sse_model::{
    RegisterSse, SendBroadcastRequest, SendToUserChannel, SendToUserRequest,
};

pub async fn send_to_user(
    state: State<AppState>,
    body: Json<SendToUserChannel>,
) -> impl IntoResponse {
    let _ = state
        .sse
        .send(SseBuilder::new(
            SseTarget::create()
                .set_user_id(body.user_id.clone())
                .set_event_name(body.event_name.clone()),
            body.0,
        ))
        .await;

    ApiResponse::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_to_user_device(
    state: State<AppState>,
    body: Json<SendToUserRequest>,
) -> impl IntoResponse {
    state
        .sse
        .send(SseBuilder::new(
            SseTarget::create()
                .set_user_id(body.user_id.clone())
                .set_device_id(body.device_id.clone())
                .set_event_name(body.event_name.clone()),
            body.0,
        ))
        .await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_broadcast(
    state: State<AppState>,
    body: Json<SendBroadcastRequest>,
) -> impl IntoResponse {
    state
        .sse
        .send(SseBuilder::new(
            SseTarget::broadcast(body.event_name.clone()),
            body.0,
        ))
        .await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn register_sse(
    state: State<AppState>,
    query: Query<RegisterSse>,
) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
    state
        .sse
        .new_client(query.user_id.clone(), query.device_id.clone())
        .await
}

pub async fn get_active_subscriber(
    state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {
    let data = state
        .sse
        .get_list_client()
        .await;
    if data.is_none() {
        return ApiResponse::failed("Tidak ditemukan subscriber".to_string());
    }

    ApiResponse::ok(data.unwrap(), "Subscriber aktif")
}
