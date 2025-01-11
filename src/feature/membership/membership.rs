use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::{
    create_object_id_option, create_or_new_object_id, QUERY_ASC, QUERY_DESC, QUERY_HIGHEST,
    QUERY_LATEST, QUERY_LOWEST, QUERY_OLDEST,
};
use crate::dto::membership_dto::MembershipDTO;
use crate::entity::membership::Membership;
use crate::feature::membership::membership_model::{
    CreateMembershipRequest, UpdateMembershipRequest,
};
use crate::translate;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_membership(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<MembershipDTO>> {
    info!(target: "membership::list", "{} trying get list membership",auth_context.claims.sub);
    if !auth_context.authorize(app::membership::READ) {
        info!(target: "membership::list", "{} is not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.name.clone().unwrap_or(default.clone());
    let date = query.date.clone().unwrap_or(default.clone());
    let price = query.price.clone().unwrap_or(default.clone());
    let mut get = Orm::get("membership");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.text().filter_string("$search", None, text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("title");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("title");
    }

    if date == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if date == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    if price == QUERY_HIGHEST.to_string() {
        get = get.group_by_desc("price");
    }

    if price == QUERY_LOWEST.to_string() {
        get = get.group_by_asc("price");
    }

    let find = get
        .and()
        .filter_bool("deleted", None, false)
        .join_one("account", "created_by_id", "_id", "created_by")
        .pageable::<MembershipDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    info!(target: "membership::list", "success get list");
    ApiResponse::ok(
        find.unwrap(),
        translate!("membership.list.success", lang).as_str(),
    )
}

pub async fn get_detail_membership(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(membership_id): Path<String>,
) -> ApiResponse<MembershipDTO> {
    info!(target: "membership::detail", "{} trying get detail membership",auth_context.claims.sub);
    if !auth_context.authorize(app::membership::READ) {
        info!(target: "membership::detail", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    let membership_id = create_object_id_option(membership_id.as_str());
    if membership_id.is_none() {
        info!(target: "membership::detail", "failed to create id");
        return ApiResponse::not_found(translate!("membership.not-found", lang).as_str());
    }
    let membership_id = membership_id.unwrap();
    let find_membership = Orm::get("membership")
        .join_one("account", "created_by_id", "_id", "created_by")
        .filter_object_id("_id", &membership_id)
        .one::<MembershipDTO>(&state.db)
        .await;

    if find_membership.is_err() {
        info!(target: "membership::detail", "membership not found");
        return ApiResponse::not_found(translate!("membership.not-found", lang).as_str());
    }
    ApiResponse::ok(
        find_membership.unwrap(),
        translate!("membership.found", lang).as_str(),
    )
}

pub async fn create_membership(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateMembershipRequest>,
) -> ApiResponse<MembershipDTO> {
    info!(target: "membership::create", "{} trying to create new membership",auth_context.claims.sub);
    if !auth_context.authorize(app::membership::CREATE) {
        info!(target: "membership::create", "{} not permitted.",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        info!(target: "membership::create", "validation failed {}.",validate.clone().unwrap_err());
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let current_time = DateTime::now();
    let membership_id = ObjectId::new();
    let membership = Membership {
        id: Some(membership_id.clone()),
        branch_id: auth_context.branch_id,
        name: body.name.clone(),
        price: body.price,
        quota: body.quota,
        created_by_id: auth_context.user_id,
        created_at: current_time.clone(),
        updated_at: current_time,
        deleted: false,
    };

    let save = Orm::insert("membership").one(&membership, &state.db).await;

    if save.is_err() {
        info!(target: "membership::create", "failed to save membership {}",save.unwrap_err());
        return ApiResponse::failed(translate!("membership.create.failed", lang).as_str());
    }
    let create_discount_id = save.unwrap();

    info!(target: "membership::create", "created membership {}",create_discount_id);
    ApiResponse::ok(
        membership.to_dto(),
        translate!("membership.create.success", lang).as_str(),
    )
}

pub async fn update_membership(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(membership_id): Path<String>,
    Json(body): Json<UpdateMembershipRequest>,
) -> ApiResponse<MembershipDTO> {
    if !auth_context.authorize(app::membership::UPDATE) {
        info!(target: "membership::update", "Failed to create new membership because user does not permitted.");
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }
    let membership_id = create_or_new_object_id(membership_id.as_str());

    let find_membership = Orm::get("membership")
        .filter_object_id("_id", &membership_id.unwrap())
        .one::<Membership>(&state.db)
        .await;

    if find_membership.is_err() {
        info!(target: "membership::update", "membership not found");
        return ApiResponse::not_found(translate!("membership.not-found", lang).as_str());
    }
    let mut membership = find_membership.unwrap();

    let mut update = Orm::update("membership");
    if body.name.is_some() {
        membership.name = body.name.clone().unwrap();
        update = update.set_str("name", body.name.clone().unwrap().as_str());
    }

    if body.price.is_some() {
        membership.price = body.price.clone().unwrap();
        update = update.set_float("price", &body.price.clone().unwrap());
    }

    if body.quota.is_some() {
        membership.quota = body.quota.clone().unwrap();
        update = update.set_number("quota", &body.quota.clone().unwrap());
    }

    let save = update
        .filter_object_id("_id", &membership_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .one(&membership, &state.db)
        .await;

    if save.is_err() {
        info!(target: "membership::update", "failed to save membership {}",save.unwrap_err());
        return ApiResponse::failed(translate!("membership.update.failed", lang).as_str());
    }
    info!(target: "membership::update","success updating membership");
    ApiResponse::ok(
        membership.to_dto(),
        translate!("membership.update.success", lang).as_str(),
    )
}

pub async fn delete_membership(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(membership_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "membership::delete", "{} trying delete  membership",auth_context.claims.sub);
    if !auth_context.authorize(app::membership::DELETE) {
        info!(target: "membership::delete", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let create_id = create_object_id_option(membership_id.as_str());
    if create_id.is_none() {
        info!(target: "membership::delete", "failed create ObjectId");
        return ApiResponse::not_found(translate!("membership.not_found", lang).as_str());
    }
    let update = Orm::update("membership")
        .set_bool("deleted", true)
        .filter_object_id("_id", &create_id.unwrap())
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        info!(target: "membership::delete","failed updating membership");
        return ApiResponse::failed(translate!("membership.delete.failed").as_str());
    }
    info!(target: "membership::delete","success deleted membership");
    ApiResponse::ok(
        "OK".to_string(),
        translate!("membership.delete.success", lang).as_str(),
    )
}
