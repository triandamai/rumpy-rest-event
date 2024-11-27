use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::create_or_new_object_id;
use crate::dto::branch_dto::BranchDTO;
use crate::entity::branch::Branch;
use crate::feature::branch::branch_model::{CreateBranchRequest, UpdateBranchRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<BranchDTO>> {
    if !auth_context.authorize("app::branch::read") {
        return ApiResponse::un_authorized(translate!("",lang.get()).as_str());
    }
    let find_all_branch = Orm::get("branch")
        .join_one("account","user_id","_id","owner")
        .pageable::<BranchDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("", lang.get()).as_str(),
    )
}

pub async fn create_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<CreateBranchRequest>,
) -> ApiResponse<Branch> {
    info!(target: "branch::create", "{} trying to create new branch",auth_context.claims.sub);
    if auth_context.authorize("app::branch::write") {
        info!(target: "branch::create", "Failed to create new branch because user does not permitted.");
        return ApiResponse::un_authorized(translate!("unauthorized", lang.get()).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        info!(target: "branch::create", "validation failed {}.",validate.clone().unwrap_err());
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation", lang.get()).as_str(),
        );
    }

    let current_branch = DateTime::now();
    let mut branch = Branch {
        id: Some(ObjectId::new()),
        branch_name: body.branch_name.clone(),
        branch_description: body.branch_description.clone(),
        branch_email: body.branch_email.clone(),
        branch_phone_number: body.branch_phone_number.clone(),
        branch_address: body.branch_address.clone(),
        branch_owner: create_or_new_object_id(body.branch_owner.clone().as_str()),
        created_at: current_branch.clone(),
        updated_at: current_branch,
    };

    let save = Orm::insert("branch").one(&branch, &state.db).await;

    if save.is_err() {
        info!(target: "branch::create", "failed to save branch {}",save.unwrap_err());
        return ApiResponse::failed(translate!("", lang.get()).as_str());
    }
    let saved_branch_id = save.unwrap();
    branch.id = Some(saved_branch_id);

    info!(target: "branch::create", "created branch {}",saved_branch_id);
    ApiResponse::ok(branch, translate!("", lang.get()).as_str())
}

pub async fn update_branch(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(branch_id): Path<String>,
    body: Json<UpdateBranchRequest>,
) -> ApiResponse<Branch> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation", lang.get()).as_str(),
        );
    }
    let branch_id = create_or_new_object_id(branch_id.as_str());

    let find_branch = Orm::get("branch")
        .filter_object_id("_id", &branch_id.unwrap())
        .one::<Branch>(&state.db)
        .await;

    if find_branch.is_err() {
        info!(target: "branch::create", "branch not found");
        return ApiResponse::not_found(translate!("not.found", lang.get()).as_str());
    }
    let mut branch = find_branch.unwrap();

    if !auth_context.branch_id.eq(&branch.branch_owner) {
        if !auth_context.branch_id.eq(&branch_id) {
            if !auth_context.authorize("app::branch::write") {
                return ApiResponse::un_authorized(translate!("unauthorized", lang.get()).as_str());
            }
        }
    }

    branch.branch_name = body.branch_name.clone();
    branch.branch_description = body.branch_description.clone();
    branch.branch_email = body.branch_email.clone();
    branch.branch_phone_number = body.branch_phone_number.clone();
    branch.branch_address = body.branch_address.clone();
    branch.updated_at = DateTime::now();

    let update = Orm::update("branch").one(&branch, &state.db).await;

    if update.is_err() {
        info!(target: "branch::create", "failed to save branch {}",update.unwrap_err());
        return ApiResponse::failed(translate!("", lang.get()).as_str());
    }

    ApiResponse::ok(branch, translate!("", lang.get()).as_str())
}
