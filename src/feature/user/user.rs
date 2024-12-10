use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::{QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST};
use crate::dto::account_dto::{AccountDTO, AccountDetailDTO};
use crate::entity::account::Account;
use crate::feature::user::user_model::CreateUserRequest;
use crate::translate;
use axum::extract::{Path, Query, State};
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;
use validator::Validate;

pub async fn get_list_user(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<AccountDetailDTO>> {
    if !auth_context.authorize("app::account::read") {
        return ApiResponse::un_authorized(translate!("", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default);
    let mut get = Orm::get("account");
    if filter.eq(QUERY_ASC) {
        get = get.group_by_asc("full_name");
    }

    if filter.eq(QUERY_DESC) {
        get = get.group_by_asc("full_name");
    }

    if filter.eq(QUERY_LATEST) {
        get = get.group_by_desc("created_at");
    }

    if filter.eq(QUERY_OLDEST) {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .join_one("account", "reply_to", "_id", "report")
        .join_one("branch", "_id", "branch_id", "branch")
        .pageable::<AccountDetailDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    ApiResponse::ok(find_all_branch.unwrap(), translate!("", lang).as_str())
}

pub async fn get_detail_user(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
) -> ApiResponse<AccountDetailDTO> {
    if !auth_context.authorize("app::account::read") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let find_user = Orm::get("account")
        .join_one("account", "reply_to", "_id", "report")
        .join_one("branch", "_id", "branch_id", "branch")
        .one::<AccountDetailDTO>(&state.db)
        .await;

    if find_user.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }

    ApiResponse::un_authorized(translate!("unauthorized", lang).as_str())
}

pub async fn create_user(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    body: Json<CreateUserRequest>,
) -> ApiResponse<AccountDTO> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(validate.unwrap_err(), translate!("", lang).as_str());
    }

    let account = Account {
        id: Some(ObjectId::new()),
        full_name: body.full_name.clone(),
        email: body.email.clone(),
        password: "".to_string(),
        gender: body.gender.clone(),
        job_title: body.job_title.clone(),
        report_to: None,
        branch_id: auth_context.branch_id,
        date_of_birth: None,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let _save = Orm::insert("account").one(&account, &state.db).await;

    ApiResponse::ok(account.to_dto(), translate!("", lang).as_str())
}
