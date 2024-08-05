use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Locale, NaiveDateTime};
use log::info;
use redis::{Commands, RedisResult};

use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;
use crate::common::sse::sse_builder::{SseBuilder, SseTarget};
use crate::entity::conversation::{ConversationType};
use crate::entity::message::{Message, MessageType};
use crate::feature::auth::auth_model::{USER_ID_KEY, USER_UUID_KEY};
use crate::feature::conversation::conversation_model::{CreateDirectConversationRequest, MessagePagination, SendMessageResponse, SendTextChatRequest};
use crate::repositories::conversation_repository;

pub async fn get_conversations(
    mut state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {
    let session_key = state
        .redis
        .create_key_sign_in_session(auth.sub.as_str());

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal mengirim chat, sesi sudah habis".to_string());
    }

    let session = get_session_from_redis.unwrap();

    let user_id = session.get(USER_ID_KEY)
        .unwrap_or(&String::from("-1"))
        .parse::<i32>().unwrap_or(-1);

    if user_id < 0 {
        return ApiResponse::failed("Gagal membuat room, sesi tidak ditemukan".to_string());
    }

    let conversations = conversation_repository::get_conversations_by_user_id(
        user_id,
        &state.postgres,
    ).await;

    ApiResponse::ok(conversations, "Daftar Percakapan")
}

pub async fn get_messages(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(id_conversation): Path<i32>,
    query: Query<MessagePagination>,
) -> impl IntoResponse {
    let session_key = state
        .redis
        .create_key_sign_in_session(auth.sub.as_str());

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal mengambil chat, sesi sudah habis".to_string());
    }

    let session = get_session_from_redis.unwrap();

    let user_id = session.get(USER_ID_KEY).unwrap_or(&String::from("-1"))
        .parse::<i32>().unwrap_or(-1);

    if user_id < 0 {
        return ApiResponse::failed("Gagal mengambil chat, sesi tidak ditemukan".to_string());
    }


    let from = NaiveDateTime::from(query.from);
    let to = NaiveDateTime::from(query.to);
    info!(target: "conversation","{} {}",from,to);
    let messages = conversation_repository::get_messages_by_conversation_id(
        id_conversation,
        from.clone(),
        to.clone(),
        &state.postgres,
    ).await;

    ApiResponse::ok(messages, "Pesan dari percakapan")
}

pub async fn create_direct_conversation(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateDirectConversationRequest>,
) -> impl IntoResponse {
    let session_key = state
        .redis
        .create_key_sign_in_session(auth.sub.as_str());

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal mengirim chat, sesi sudah habis".to_string());
    }

    let session = get_session_from_redis.unwrap();

    let user_id = session.get(USER_ID_KEY).unwrap_or(&String::from("-1"))
        .parse::<i32>().unwrap_or(-1);

    if user_id < 0 {
        return ApiResponse::failed("Gagal membuat room, sesi tidak ditemukan".to_string());
    }

    let search_conversation =
        conversation_repository::search_direct_conversation_if_exists(
            body.members.clone(),
            &state.postgres,
        ).await;


    if search_conversation.len() > 0 {
        let conversation = search_conversation
            .get(0)
            .unwrap()
            .conversation.clone();
        return ApiResponse::create(
            1000,
            conversation,
            "Conversation sudah ada",
        );
    }

    let saved_conversation = conversation_repository::create_conversation(
        ConversationType::Direct,
        body.members.clone(),
        &state.postgres,
    ).await;

    if saved_conversation.is_none() {
        return ApiResponse::failed("Gagal membuat room".to_string());
    }
    return ApiResponse::ok(
        saved_conversation.unwrap(), "Berhasil membuat room",
    );
}

pub async fn send_text_message(
    mut state: State<AppState>,
    claims: JwtClaims,
    body: Json<SendTextChatRequest>,
) -> impl IntoResponse {
    let session_key = state
        .redis
        .create_key_sign_in_session(claims.sub.as_str());

    let get_session_from_redis: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if get_session_from_redis.is_err() {
        return ApiResponse::failed("Gagal mengirim chat, sesi sudah habis".to_string());
    }

    let session = get_session_from_redis.unwrap();
    let user_id = session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("-1"))
        .parse::<i32>()
        .unwrap_or(-1);

    let user_uuid = session
        .get(USER_UUID_KEY)
        .unwrap_or(&String::from(""))
        .to_string();

    if user_id < 0 {
        return ApiResponse::failed("Gagal mengirim chat, sesi sudah habis".to_string());
    }

    //uuid v4 length is 32+(v)4' = 36
    if user_uuid.chars().count() < 32 {
        return ApiResponse::failed("Gagal mengirim chat, sesi sudah habis".to_string());
    }

    let find_conversation_with_members = conversation_repository::get_conversation_with_member(
        body.conversation_id,
        &state.postgres,
    ).await;

    if find_conversation_with_members.is_none() {
        return ApiResponse::failed("Gagal megirim chat".to_string());
    }
    let conversation = find_conversation_with_members.unwrap();

    let current_date = chrono::Utc::now().naive_local();
    let message = Message {
        id: Default::default(),
        conversation_id: body.conversation_id.clone(),
        sender_id: user_id,
        message_content: body.content.clone(),
        message_type: MessageType::Text,
        message_sent_at: current_date,
        updated_at: current_date,
    };

    let saved_message = conversation_repository::create_message(message, &state.postgres)
        .await;

    if saved_message.is_none() {
        return ApiResponse::failed("Failed to send chat".to_string());
    }
    let message = saved_message.unwrap();

    let payload = SendMessageResponse {
        conversation: conversation.clone(),
        message: message.clone(),
    };

    for member in conversation.members {
        let _ = state
            .sse
            .send(
                SseBuilder::new(
                    SseTarget::create()
                        .set_user_id(member.account.uuid)
                        .set_event_name("new_chat".to_string()),
                    payload.clone(),
                )
            )
            .await;
    }

    ApiResponse::ok(payload, "Chat berhasil dikirim")
}
