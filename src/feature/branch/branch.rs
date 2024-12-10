use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::{
    create_object_id_option, create_or_new_object_id, QUERY_ASC, QUERY_DESC, QUERY_HIGHEST,
    QUERY_LATEST, QUERY_OLDEST,
};
use crate::dto::branch_dto::BranchDTO;
use crate::entity::branch::Branch;
use crate::feature::auth::auth_model::USER_ID_KEY;
use crate::feature::branch::branch_model::{CreateBranchRequest, UpdateBranchRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_extra::handler::Or;
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use serde::de::Unexpected::Str;
use validator::Validate;

pub async fn get_list_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<BranchDTO>> {
    if !auth_context.authorize("app::branch::read") {
        return ApiResponse::un_authorized(translate!("", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default);
    let mut get = Orm::get("branch");

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("branch_name");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("branch_name");
    }

    if filter == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if filter == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .join_one("account", "branch_owner", "_id", "owner")
        .filter_bool("deleted",None,false)
        .pageable::<BranchDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    ApiResponse::ok(find_all_branch.unwrap(), translate!("", lang).as_str())
}

pub async fn get_detail_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(branch_id): Path<String>,
) -> ApiResponse<BranchDTO> {
    if !auth_context.authorize("app::branch::read") {
        return ApiResponse::un_authorized(translate!("", lang).as_str());
    }
    let branch_id = ObjectId::parse_str(branch_id.as_str());
    if branch_id.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }
    let branch_id = branch_id.unwrap();
    let find_all_branch = Orm::get("branch")
        .filter_object_id("_id", &branch_id)
        .one::<BranchDTO>(&state.db)
        .await;

    if find_all_branch.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }
    ApiResponse::ok(find_all_branch.unwrap(), translate!("", lang).as_str())
}

pub async fn create_branch(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<CreateBranchRequest>,
) -> ApiResponse<BranchDTO> {
    info!(target: "branch::create", "{} trying to create new branch",auth_context.claims.sub);

    if !auth_context.authorize("app::branch::write") {
        info!(target: "branch::create", "Failed to create new branch because user does not permitted.");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        info!(target: "branch::create", "validation failed {}.",validate.clone().unwrap_err());
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation", lang).as_str(),
        );
    }

    let get_session = state
        .redis
        .get_session_sign_in(auth_context.claims.sub.clone().as_str());

    if get_session.is_err() {
        let err = get_session.unwrap_err();
        info!(target: "branch::create", "get_session failed {}.",err);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    let session = get_session.unwrap();
    let user_id = session
        .get(USER_ID_KEY)
        .map_or(None, |value| create_or_new_object_id(value));

    let current_branch = DateTime::now();
    let branch_id = ObjectId::new();
    let mut branch = Branch {
        id: Some(branch_id.clone()),
        branch_name: body.branch_name.clone(),
        branch_description: body.branch_description.clone(),
        branch_email: body.branch_email.clone(),
        branch_phone_number: body.branch_phone_number.clone(),
        branch_address: body.branch_address.clone(),
        branch_owner: user_id,
        created_at: current_branch.clone(),
        updated_at: current_branch,
        deleted: false,
    };

    let save = Orm::insert("branch").one(&branch, &state.db).await;

    if save.is_err() {
        info!(target: "branch::create", "failed to save branch {}",save.unwrap_err());
        return ApiResponse::failed(translate!("branch.insert.failed", lang).as_str());
    }
    let saved_branch_id = save.unwrap();

    info!(target: "branch::create", "created branch {}",saved_branch_id);
    ApiResponse::ok(
        branch.to_dto(),
        translate!("branch.insert.success", lang).as_str(),
    )
}

pub async fn update_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(branch_id): Path<String>,
    body: Json<UpdateBranchRequest>,
) -> ApiResponse<BranchDTO> {
    if !auth_context.authorize("app::branch::write") {
        info!(target: "branch::create", "Failed to create new branch because user does not permitted.");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation", lang).as_str(),
        );
    }
    let branch_id = create_or_new_object_id(branch_id.as_str());

    let find_branch = Orm::get("branch")
        .filter_object_id("_id", &branch_id.unwrap())
        .one::<Branch>(&state.db)
        .await;

    if find_branch.is_err() {
        info!(target: "branch::create", "branch not found");
        return ApiResponse::not_found(translate!("branch.not-found", lang).as_str());
    }
    let mut branch = find_branch.unwrap();

    if !auth_context.branch_id.eq(&branch.branch_owner) {
        if !auth_context.branch_id.eq(&branch_id) {
            if !auth_context.authorize("app::branch::write") {
                return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
            }
        }
    }

    branch.branch_name = body.branch_name.clone();
    branch.branch_description = body.branch_description.clone();
    branch.branch_email = body.branch_email.clone();
    branch.branch_phone_number = body.branch_phone_number.clone();
    branch.branch_address = body.branch_address.clone();
    branch.updated_at = DateTime::now();

    let update = Orm::update("branch")
        .filter_object_id("_id",&branch_id.unwrap()).one(&branch, &state.db).await;

    if update.is_err() {
        info!(target: "branch::create", "failed to save branch {}",update.unwrap_err());
        return ApiResponse::failed(translate!("branch.update.failed", lang).as_str());
    }

    ApiResponse::ok(
        branch.to_dto(),
        translate!("branch.update.success", lang).as_str(),
    )
}

pub async fn delete_branch(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(branch_id): Path<String>,
) -> ApiResponse<String> {
    if !auth_context.authorize("app::branch::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let create_id = create_object_id_option(branch_id.as_str());
    if create_id.is_none() {
        return ApiResponse::not_found(translate!("branch.not_found", lang).as_str());
    }
    let update = Orm::update("branch")
        .set_bool("deleted", true)
        .filter_object_id("_id", &create_id.unwrap())
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        info!(target: "branch::delete", "failed to delete branch {}",update.unwrap_err());
        return ApiResponse::failed(translate!("branch.delete.failed").as_str());
    }

    ApiResponse::ok(
        "OK".to_string(),
        translate!("branch.delete.success", lang).as_str(),
    )
}
