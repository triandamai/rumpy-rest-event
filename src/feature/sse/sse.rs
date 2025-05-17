use std::convert::Infallible;

use axum::Json;
use axum::extract::{Query, State};
use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use futures::Stream;

use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::{AuthContext, JwtClaims};
use crate::common::sse::sse_builder::{SseBuilder, SseTarget};
use crate::feature::sse::sse_model::{RegisterPublicSse, RegisterSse, SendBroadcastRequest, SendToUserChannel, SendToUserRequest, SubscribeToTopicRequest};

pub async fn send_to_user(
    state: State<AppState>,
    Json(body): Json<SendToUserChannel>,
) -> ApiResponse<String> {
    let _ = state
        .sse
        .send(SseBuilder::new(
            SseTarget::create()
                .set_user_id(body.user_id.clone())
                .set_event_name(body.event_name.clone()),
            body,
        ))
        .await;

    ApiResponse::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_to_user_device(
    state: State<AppState>,
    Json(body): Json<SendToUserRequest>,
) -> ApiResponse<String> {
    state
        .sse
        .send(SseBuilder::new(
            SseTarget::create()
                .set_user_id(body.user_id.clone())
                .set_device_id(body.device_id.clone())
                .set_event_name(body.event_name.clone()),
            body,
        ))
        .await;

    ApiResponse::<String>::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn send_broadcast(
    state: State<AppState>,
    Json(body): Json<SendBroadcastRequest>,
) -> ApiResponse<String> {
    state
        .sse
        .send(SseBuilder::new(
            SseTarget::broadcast(body.event_name.clone()),
            body,
        ))
        .await;

    ApiResponse::ok("subscribe".to_string(), "Berhasil mengirimkan ke topic")
}

pub async fn register_sse(
    state: State<AppState>,
    auth_context: AuthContext,
    query: Query<RegisterSse>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let user_id = auth_context.get_user_id_as_string();

    state
        .sse
        .new_client(user_id, query.device_id.clone())
        .await
}

pub async fn register_public_sse(
    state: State<AppState>,
    query: Query<RegisterPublicSse>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    state
        .sse
        .new_client(query.user_id.clone(), query.device_id.clone())
        .await
}

pub async fn subscribe_to_topic(
    mut state: State<AppState>,
    body: Json<SubscribeToTopicRequest>,
) -> ApiResponse<String> {
    let subscribe = state
        .redis
        .subscribe_to_topic(body.topic.clone(), body.user_id.clone());

    subscribe.map_or_else(
        |e| ApiResponse::failed(e.as_str()),
        |message| ApiResponse::ok(message, "Berhasil subscribe ke topic"),
    )
}

pub async fn unsubscribe_to_topic(
    mut state: State<AppState>,
    body: Json<SubscribeToTopicRequest>,
) -> ApiResponse<String> {
    let subscribe = state
        .redis
        .subscribe_to_topic(body.topic.clone(), body.user_id.clone());

    subscribe.map_or_else(
        |e| ApiResponse::failed(e.as_str()),
        |message| ApiResponse::ok(message, "Berhasil unsubscribe ke topic"),
    )
}

pub async fn get_active_subscriber(state: State<AppState>, _: JwtClaims) -> impl IntoResponse {
    let data = state.sse.get_list_client().await;
    if data.is_none() {
        return ApiResponse::failed("Tidak ditemukan subscriber");
    }

    ApiResponse::ok(data.unwrap(), "Subscriber aktif")
}
