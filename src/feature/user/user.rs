use super::user_model::{CreateNewUserRequest, SearchByUsernameRequest};
use crate::common::api_response::PagingResponse;
use crate::common::{
    api_response::{ApiResponse, PaginationRequest},
    app_state::AppState,
    jwt::JwtClaims,
};
use crate::entity::user_credential::{UserCredential, UserDTO};
use axum::{
    extract::{Query, State}
};
use mongodb::bson::doc;

pub async fn get_list_user_with_paging(
    mut state: State<AppState>,
    _auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<UserDTO>> {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let paging = UserCredential::find_with_paging(doc! {}, page, size, &state.db)
        .await;

    ApiResponse::ok(paging, "Data user")
}

pub async fn get_user_by_username(
    mut state: State<AppState>,
    _auth: JwtClaims,
    query: Query<SearchByUsernameRequest>,
) -> ApiResponse<UserCredential> {
    if query.username.is_none() {
        return ApiResponse::not_found("username kosong.".to_string());
    }
    let username = query.username.clone().unwrap();

    let find_user = UserCredential::find_one(doc! {
        "username": doc!{
            "$eq":username
        }
    }, &state.db).await;
    if find_user.is_none() {
        return ApiResponse::not_found("Tidak dapat menemukan user.".to_string());
    }
    let find_user = find_user.unwrap();
    ApiResponse::ok(find_user, "Data user")
}
