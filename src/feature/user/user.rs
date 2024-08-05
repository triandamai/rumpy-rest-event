use std::collections::HashMap;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use log::info;

use crate::common::{
    api_response::{ApiResponse, PaginationRequest},
    app_state::AppState,
    jwt::JwtClaims,
};

use super::user_model::{CreateNewUserRequest, SearchByUsernameRequest};
use crate::entity::user_credential::{AuthProvider, UserCredential, UserStatus};
use crate::repositories::{auth_repository, user_repository};
use bcrypt::{BcryptResult, DEFAULT_COST};
use redis::{Commands, RedisResult};
use crate::common::redis::RedisClient;


pub async fn get_list_user_with_paging(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state.redis.client.hgetall(session_key);
    if find_session.is_err() {
        info!(target: "get_list_user_with_paging","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Akses ditolak");
    }

    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let data = user_repository::get_list_user(page, size, &state.postgres).await;

    ApiResponse::ok(data, "Data user")
}


pub async fn get_user_by_username(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<SearchByUsernameRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(auth.sub.as_str());

    let find_session:RedisResult<HashMap<String, String>> = state.redis.client.hgetall(session_key);
    if find_session.is_err() {
        info!(target: "get_list_user_with_paging","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Akses ditolak");
    }
    if query.username.is_none() {
        return ApiResponse::failed("Tidak dapat mencari user tanpa @username".to_string());
    }
    let find_user = user_repository::get_user_by_username(
        &query.username.clone().unwrap(),
        &state.postgres,
    ).await;

    return ApiResponse::ok(find_user, "Data user");
}

pub async fn create_user(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateNewUserRequest>,
) -> impl IntoResponse {
    let find_existing =
        auth_repository::get_user_by_email(body.email.clone(), &state.postgres).await;

    if find_existing.is_none() {
        return ApiResponse::failed("Login gagal, akun tidak ditemukan".to_string());
    }

    let create_password: BcryptResult<String> = bcrypt::hash(body.password.clone(), DEFAULT_COST);

    if create_password.is_err() {
        return ApiResponse::failed("Gagal mendaftarkan akun, silahkan coba beberapa saat lagi".to_string());
    }
    let uuid = uuid::Uuid::new_v4();
    let status = if body.auto_confirm {
        UserStatus::WaitingConfirmation
    } else {
        UserStatus::Active
    };
    let user = UserCredential {
        id: Default::default(),
        uuid: uuid.to_string(),
        username: "n/a".to_string(),
        password: create_password.unwrap(),
        full_name: "n/a".to_string(),
        email: body.email.clone(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
        status: status,
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let saved_user = auth_repository::create_new_user(user, &state.postgres).await;

    if saved_user.is_err() {
        info!(target:"create_user","{:?}",saved_user.unwrap_err());
        return ApiResponse::failed("Gagal membuat akun kamu, coba lagi nanti.".to_string());
    }

    //todo:: send email
    let user_credential = saved_user.unwrap();

    ApiResponse::ok(
        user_credential,
        "User has been created",
    )
}
