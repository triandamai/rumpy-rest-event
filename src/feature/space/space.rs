use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use log::info;
use redis::{Commands, RedisResult};
use crate::common::api_response::{ApiResponse, PaginationRequest};
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;
use crate::entity::space::{Space, SpaceFollower};
use crate::feature::auth::auth_model::USER_ID_KEY;
use crate::feature::space::space_model::{CreateSpaceRequest};
use crate::repositories::space_repository;

pub async fn get_list_space_with_pagination(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "get_list_space_with_pagination","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }

    let data = space_repository::get_list_space_with_paging(
        query.page.unwrap(),
        query.size.unwrap(),
        &state.postgres,
    ).await;

    ApiResponse::ok(data, "Data space")
}

pub async fn get_list_following_space_current_user(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "get_list_following_space_current_user","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }
    let session = find_session.unwrap();
    let user_id = session.get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let data = space_repository::get_list_space_by_current_user(
        query.page.unwrap(),
        query.size.unwrap(),
        user_id,
        &state.postgres,
    ).await;

    ApiResponse::ok(data, "Data space kamu")
}

pub async fn get_detail_space(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(space_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "create_space","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }

    let find_space = space_repository::get_space_by_id(
        space_id,
        &state.postgres,
    ).await;

    if find_space.is_none() {
        info!(target: "create_space","{:?}",find_session.unwrap_err());
        return ApiResponse::failed("Tidak menemukan space".to_string());
    }

    ApiResponse::ok(find_space.unwrap(), "Detail space")
}

pub async fn create_space(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateSpaceRequest>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "create_space","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }
    let session = find_session.unwrap();
    let user_id = session.get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let space = Space {
        id: 0,
        user_id: user_id,
        name: body.name.clone(),
        space_thumbnail_id: Some(body.space_thumbnail_id.unwrap()),
        is_public: body.is_public,
        description: body.description.clone(),
        created_at: Default::default(),
    };

    let saved = space_repository::create_space(
        &space,
        &state.postgres,
    ).await;

    if saved.is_none() {
        info!(target: "create_space","Gagal membuat space")
    }

    return ApiResponse::created(saved, "Berhasil membuat space mu");
}

pub async fn update_space(
    mut state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {
    ApiResponse::<String>::failed("belum implementasi".to_string())
}

pub async fn update_thumbnail_space(
    mut state: State<AppState>,
    auth: JwtClaims,
) -> impl IntoResponse {
    ApiResponse::<String>::failed("belum implementasi".to_string())
}

pub async fn delete_space(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(space_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "delete_space","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }
    let session = find_session.unwrap();
    let user_id = session.get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let find_space = space_repository::get_space_by_id(
        space_id.clone(),
        &state.postgres,
    ).await;


    if find_space.is_none() {
        info!(target: "delete_space","Cannot find");
        return ApiResponse::failed("Failed to create space".to_string());
    }
    let space = find_space.unwrap();

    if space.user_id != user_id {
        info!(target: "delete_space","trying to delete space, but user not the owner");
        return ApiResponse::failed("Cannot delete space, because you are not the owner".to_string());
    }

    let delete_space = space_repository::delete_space(
        space_id,
        user_id,
        &state.postgres,
    ).await;

    if delete_space.is_err() {
        let err = delete_space.unwrap_err();
        let message = err.clone().to_string();
        info!(target: "delete_space","{:?}",err.clone());
        return ApiResponse::failed(message);
    }
    return ApiResponse::ok(delete_space.ok(), "Berhasil menghapus space");
}

pub async fn follow_space(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(space_id):Path<i32>,
) -> impl IntoResponse {
    let session_key = state.redis
        .create_key_sign_in_session(auth.sub.as_str());

    let find_session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if find_session.is_err() {
        info!(target: "follow_space","{:?}",find_session.unwrap_err());
        return ApiResponse::un_authorized("Kamu tidak memiliki akses");
    }
    let session = find_session.unwrap();
    let user_id = session.get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let find_space = space_repository::get_space_by_id(
        space_id,
        &state.postgres,
    ).await;

    if find_space.is_none() {
        info!(target: "delete_space","Cannot found data");
        return return ApiResponse::failed("Failed to create space".to_string());
    }
    let space = find_space.unwrap();


    let saved = space_repository::follow_space(
        SpaceFollower {
            space_id: space.id,
            user_id: user_id.clone(),
            created_at: Default::default(),
        },
        &state.postgres,
    ).await;

    if saved.is_err() {
        info!(target: "follow_space","{:?}",saved.unwrap_err());
        return ApiResponse::failed("Gagal memfollow".to_string());
    }

    return ApiResponse::created(saved.unwrap(), "Behasil memfollow");
}