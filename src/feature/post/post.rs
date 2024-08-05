use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use log::info;
use redis::{Commands, RedisResult};
use crate::common::api_response::{ApiResponse, PaginationRequest};
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;
use crate::entity::post::{Post, PostComment};
use crate::feature::auth::auth_model::USER_ID_KEY;
use crate::feature::post::post_model::{CreatePostCommentRequest, CreatePostRequest, CreateReplyPostCommentRequest};
use crate::repositories::post_repository;

pub async fn get_list_post_with_paging(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }
    let data = post_repository::get_list_post(
        query.page.unwrap_or(1),
        query.size.unwrap_or(10),
        &state.postgres,
    ).await;

    return ApiResponse::ok(data, "Data Post");
}

pub async fn get_list_post_by_current_user(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_by_current_user::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }

    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let data = post_repository::get_list_post_by_current_user(
        query.page.unwrap_or(1),
        query.size.unwrap_or(10),
        user_id,
        &state.postgres,
    ).await;

    return ApiResponse::ok(data, "Your post");
}

pub async fn get_detail_post(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }

    let post = post_repository::get_post_by_id(
        post_id.clone(),
        &state.postgres,
    ).await;

    if post.is_err() {
        info!(target: "get_list_post_with_paging::controller","Not Found");
        return ApiResponse::failed(post.unwrap_err());
    }
    ApiResponse::ok(post, "Retrieve detail post")
}

pub async fn get_list_post_by_space(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(space_id): Path<(i32)>,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }
    let data = post_repository::get_list_post_by_space(
        query.page.unwrap_or(1),
        query.size.unwrap_or(10),
        space_id,
        &state.postgres,
    ).await;

    return ApiResponse::ok(data, "Data post by space");
}

pub async fn create_post(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreatePostRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }

    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let post = Post {
        id: 0,
        user_id: user_id,
        space_id: body.space_id,
        body: body.body.clone(),
        post_type: body.post_type.clone(),
        comments: 0,
        watch: 0,
        up_vote: 0,
        down_vote: 0,
        share_count: 0,
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let saved_post = post_repository::create_post(
        post,
        &state.postgres,
    ).await;

    if saved_post.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed(saved_post.unwrap_err());
    }

    ApiResponse::created(saved_post.unwrap(), "belum implementasi")
}

pub async fn delete_post(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let find_post = post_repository::get_post_by_id(
        post_id,
        &state.postgres,
    ).await;

    if find_post.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Gagal menghapus post.".to_string());
    }

    let post = find_post.unwrap();
    if post.user_id != user_id {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Hanya owner yang dapat menghapus.".to_string());
    }

    let delete_post = post_repository::delete_post(
        post_id,
        &state.postgres,
    ).await;

    if delete_post.is_err() {
        info!(target: "get_list_post_with_paging::controller","{:?}",delete_post.clone().unwrap_err());
        return ApiResponse::failed(delete_post.unwrap_err());
    }
    ApiResponse::ok(delete_post.unwrap(), "Berhasil meghapus post")
}

pub async fn send_watch(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_like::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_watch(
        post_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_like::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}

pub async fn send_up_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_like::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_up_vote(
        post_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_like::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}

pub async fn send_down_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_like::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_down_vote(
        post_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_like::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}

pub async fn get_list_comment_by_post(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(post_id): Path<(i32)>,
    query: Query<PaginationRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }

    let get_comments = post_repository::get_list_comment(
        query.page.unwrap_or(1),
        query.size.unwrap_or(10),
        post_id,
        &state.postgres,
    ).await;

    ApiResponse::ok(get_comments, "Data comment")
}

pub async fn send_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreatePostCommentRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let comment = PostComment {
        id: 0,
        user_id: user_id,
        post_id: body.post_id,
        reply_to_id: None,
        body: body.body.clone(),
        reply_count: 0,
        watch: 0,
        up_vote: 0,
        down_vote: 0,
        created_at: Default::default(),
    };

    let saved = post_repository::send_comment(
        comment,
        &state.postgres,
    ).await;

    if saved.is_err() {
        return ApiResponse::failed(saved.unwrap_err());
    }
    ApiResponse::ok(saved.unwrap(), "Komen dikirim")
}

pub async fn send_reply_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateReplyPostCommentRequest>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "get_list_post_with_paging::controller","Session null");
        return ApiResponse::failed("Kamu tidak memiliki akses.".to_string());
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let comment = PostComment {
        id: 0,
        user_id: user_id,
        post_id: body.post_id,
        reply_to_id: body.comment_id,
        body: body.body.clone(),
        reply_count: 0,
        watch: 0,
        up_vote: 0,
        down_vote: 0,
        created_at: Default::default(),
    };

    let saved = post_repository::send_reply_comment(
        comment,
        &state.postgres,
    ).await;

    if saved.is_err() {
        return ApiResponse::failed(saved.unwrap_err());
    }
    ApiResponse::ok(saved.unwrap(), "Komen dikirim")
}


pub async fn send_watch_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(comment_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_watch_comment::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_watch(
        comment_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_watch_comment::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}

pub async fn send_up_vote_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(comment_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_up_vote_comment::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_up_vote_comment(
        comment_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_up_vote_comment::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}

pub async fn send_down_vote_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(comment_id): Path<(i32)>,
) -> impl IntoResponse {
    let session_key = state.redis.create_key_sign_in_session(&auth.sub.clone());
    let session: RedisResult<HashMap<String, String>> = state
        .redis
        .client
        .hgetall(session_key);

    if session.is_err() {
        info!(target: "send_down_vote_comment::controller","Session null");
        return ApiResponse::un_authorized("Kamu tidak memiliki akses.");
    }
    let user_id = session.unwrap().get(USER_ID_KEY)
        .unwrap_or(&String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);

    let save_like = post_repository::send_down_vote_comment(
        comment_id,
        user_id,
        &state.postgres,
    ).await;
    if save_like.is_err() {
        info!(target: "send_down_vote_comment::controller","failed to like");
        return ApiResponse::failed(save_like.unwrap_err());
    }

    ApiResponse::created(save_like.unwrap(), "Berhasil like postingan")
}