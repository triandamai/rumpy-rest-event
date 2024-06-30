use std::convert::Infallible;

use axum::extract::{Query, State};
use axum::Json;
use axum::response::{IntoResponse, Sse};
use axum::response::sse::Event;
use futures::Stream;

use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::feature::models::event::{RegisterSse, SendBroadcastRequest, SendToUserChannel, SendToUserRequest};

pub async fn send_to_channel(
    mut state: State<AppState>,
    body: Json<SendToUserChannel>,
) -> impl IntoResponse {
   let _ = state.sse.send_to_channel(
        body.channel_id.clone(),
        body.event_name.clone(),
        &ApiResponse::<SendToUserChannel>::ok(body.0.clone(), "Data"),
    ).await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_to_user(
    state: State<AppState>,
    body: Json<SendToUserRequest>,
) -> impl IntoResponse {
    state.sse.send_to_user(
        body.channel_id.clone(),
        body.user_id.clone(),
        body.event_name.clone(),
        &ApiResponse::ok(body.message.clone(), "Mueheheh"),
    ).await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_broadcast(
    state: State<AppState>,
    body: Json<SendBroadcastRequest>,
) -> impl IntoResponse {
    state.sse.broadcast_all(
        body.event_name.clone().as_str(),
        &ApiResponse::ok(body.message.clone(), ""),
    ).await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn register_sse(
    state: State<AppState>,
    query: Query<RegisterSse>,
) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
    state.sse.new_client(
        query.channel_id.clone(),
        query.user_id.clone(),
    ).await
}