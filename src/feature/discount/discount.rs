use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::{
    create_object_id_option, create_or_new_object_id, QUERY_ASC, QUERY_DESC, QUERY_LATEST,
    QUERY_OLDEST,
};
use crate::dto::discount_dto::DiscountDTO;
use crate::entity::discount::Discount;
use crate::feature::discount::discount_model::{CreateDiscountRequest, UpdateDiscountRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_discount(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<DiscountDTO>> {
    info!(target: "discount::list", "{} trying get list discount",auth_context.claims.sub);
    if !auth_context.authorize(app::discount::READ) {
        info!(target: "discount::list", "{} is not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.name.clone().unwrap_or(default.clone());
    let mut get = Orm::get("discount");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.filter_string("$text", Some("$search"), text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("title");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("title");
    }

    if filter == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if filter == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find = get
        .join_one("account", "created_by_id", "_id", "created_by")
        .filter_bool("deleted", None, false)
        .pageable::<DiscountDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    info!(target: "discount::list", "success get list");
    ApiResponse::ok(
        find.unwrap(),
        translate!("discount.list.success", lang).as_str(),
    )
}

pub async fn get_detail_discount(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(discount_id): Path<String>,
) -> ApiResponse<DiscountDTO> {
    info!(target: "discount::detail", "{} trying get detail discount",auth_context.claims.sub);
    if !auth_context.authorize(app::discount::READ) {
        info!(target: "discount::detail", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    let discount_id = create_object_id_option(discount_id.as_str());
    if discount_id.is_none() {
        info!(target: "discount::detail", "failed to create id");
        return ApiResponse::not_found(translate!("discount.not-found", lang).as_str());
    }
    let discount_id = discount_id.unwrap();
    let find_discount = Orm::get("discount")
        .join_one("account", "created_by_id", "_id", "created_by")
        .filter_object_id("_id", &discount_id)
        .one::<DiscountDTO>(&state.db)
        .await;

    if find_discount.is_err() {
        info!(target: "discount::detail", "discount not found");
        return ApiResponse::not_found(translate!("discount.not-found", lang).as_str());
    }
    ApiResponse::ok(
        find_discount.unwrap(),
        translate!("discount.found", lang).as_str(),
    )
}

pub async fn create_discount(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateDiscountRequest>,
) -> ApiResponse<DiscountDTO> {
    info!(target: "discount::create", "{} trying to create new discount",auth_context.claims.sub);
    if !auth_context.authorize(app::discount::CREATE) {
        info!(target: "branch::create", "{} not permitted.",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        info!(target: "discount::create", "validation failed {}.",validate.clone().unwrap_err());
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let current_time = DateTime::now();
    let discount_id = ObjectId::new();
    let discount = Discount {
        id: Some(discount_id.clone()),
        title: body.title.clone(),
        amount: body.amount,
        created_by_id: auth_context.user_id,
        created_at: current_time.clone(),
        updated_at: current_time,
        deleted: false,
    };

    let save = Orm::insert("discount").one(&discount, &state.db).await;

    if save.is_err() {
        info!(target: "discount::create", "failed to save discount {}",save.unwrap_err());
        return ApiResponse::failed(translate!("discount.create.failed", lang).as_str());
    }
    let create_discount_id = save.unwrap();

    info!(target: "discount::create", "created discount {}",create_discount_id);
    ApiResponse::ok(
        discount.to_dto(),
        translate!("discount.create.success", lang).as_str(),
    )
}

pub async fn update_discount(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(discount_id):Path<String>,
    Json(body): Json<UpdateDiscountRequest>
) -> ApiResponse<DiscountDTO> {

    if !auth_context.authorize(app::discount::UPDATE) {
        info!(target: "discount::update", "Failed to create new discount because user does not permitted.");
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }
    let branch_id = create_or_new_object_id(discount_id.as_str());

    let find_branch = Orm::get("discount")
        .filter_object_id("_id", &branch_id.unwrap())
        .one::<Discount>(&state.db)
        .await;

    if find_branch.is_err() {
        info!(target: "discount::update", "discount not found");
        return ApiResponse::not_found(translate!("discount.not-found", lang).as_str());
    }
    let mut discount = find_branch.unwrap();

    let mut update = Orm::update("discount");
    if body.title.is_some() {
        discount.title = body.title.clone().unwrap();
        update = update.set_str("title", body.title.clone().unwrap().as_str());
    }

    if body.amount.is_some() {
        discount.amount = body.amount.clone().unwrap();
        update = update.set_float("amount", &body.amount.clone().unwrap());
    }

    let save = update
        .filter_object_id("_id", &branch_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .one(&discount, &state.db)
        .await;

    if save.is_err() {
        info!(target: "discount::uodate", "failed to save branch {}",save.unwrap_err());
        return ApiResponse::failed(translate!("discount.update.failed", lang).as_str());
    }
    info!(target: "discount::uodate","success updating discount");
    ApiResponse::ok(
        discount.to_dto(),
        translate!("discount.update.success", lang).as_str(),
    )
}

pub async fn delete_discount(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(discount_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "discount::delete", "{} trying delete  discount",auth_context.claims.sub);
    if !auth_context.authorize(app::discount::DELETE) {
        info!(target: "discount::delete", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let create_id = create_object_id_option(discount_id.as_str());
    if create_id.is_none() {
        info!(target: "discount::delete", "failed create ObjectId");
        return ApiResponse::not_found(translate!("discount.not_found", lang).as_str());
    }
    let update = Orm::update("discount")
        .set_bool("deleted", true)
        .filter_object_id("_id", &create_id.unwrap())
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        info!(target: "discount::delete","failed updating discount");
        return ApiResponse::failed(translate!("discount.delete.failed").as_str());
    }
    info!(target: "discount::delete","success deleted discount");
    ApiResponse::ok(
        "OK".to_string(),
        translate!("discount.delete.success", lang).as_str(),
    )
}
