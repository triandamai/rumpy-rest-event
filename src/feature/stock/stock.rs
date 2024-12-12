use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::{
    create_object_id_option, QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST,
};
use crate::dto::product_dto::ProductDTO;
use crate::entity::product_log::ProductLog;
use crate::feature::stock::stock_model::UpdateStockRequest;
use crate::translate;
use axum::extract::{Query, State};
use axum::Json;
use bson::{doc, DateTime, Document};
use log::info;

pub async fn get_lis_stock(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ProductDTO>> {
    info!(target: "stock::list","{} trying to get list product stock",auth_context.claims.sub);
    if !auth_context.authorize("app::stock::read") {
        info!(target:"stock::list","{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        info!(target:"stock::list","failed to get stock id");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default.clone());
    let mut get = Orm::get("product");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.filter_string("$text", Some("$search"), text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("product_name");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("product_name");
    }

    if filter == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if filter == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by")
        .pageable::<ProductDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    info!(target: "stock::list","successfully get list stock");
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("stock.list.success", lang).as_str(),
    )
}

pub async fn update_stock(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    body: Json<UpdateStockRequest>,
) -> ApiResponse<ProductDTO> {
    info!(target: "stock::update","{} trying to update stock",auth_context.claims.sub);
    if !auth_context.authorize("app::stock::write") {
        info!(target: "stock::update","{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let product_id = create_object_id_option(body.product_id.clone().as_str());
    if product_id.is_none() {
        info!(target:"stock::update","failed to create product id");
        return ApiResponse::not_found(translate!("product.not-found", lang).as_str());
    }

    let session = state.db.start_session().await;
    if session.is_err() {
        info!(target:"stock::update","failed to create trx session");
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _start = session.start_transaction();

    let product_log = ProductLog {
        id: None,
        branch_id: None,
        description: "".to_string(),
        log_type: "".to_string(),
        stock: 0,
        created_by_id: None,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let insert_log = Orm::insert("product-log")
        .one_with_session::<ProductLog>(product_log, &state.db, &mut session)
        .await;

    if insert_log.is_err() {
        let _abort = session.abort_transaction().await;
        let err = insert_log.unwrap_err().to_string();
        info!(target:"stock::update","failed to insert product-log: {}", err);
        return ApiResponse::failed(translate!("update.failed", lang).as_str());
    }

    let update_product = Orm::update("product")
        .filter_object_id("_id", &product_id.unwrap())
        .one_with_session::<Document>(
            doc! {
                "$inc":{"product_stock":1},
                "$set":{"updated_at":DateTime::now()}
            },
            &state.db,
            &mut session,
        )
        .await;

    if update_product.is_err() {
        let _abort = session.abort_transaction().await;
        let err = insert_log.unwrap_err().to_string();
        info!(target:"stock::update","failed to insert product-log: {}", err);
        return ApiResponse::failed(translate!("update.failed", lang).as_str());
    }

    //commit
    let _commit = session.commit_transaction().await;

    let find_detail_stock = Orm::get("product")
        .filter_object_id("_id", &product_id.unwrap())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by")
        .one::<ProductDTO>(&state.db)
        .await;

    if find_detail_stock.is_err() {
        let err = find_detail_stock.unwrap_err().to_string();
        info!(target: "stock::update","{}", err);
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    info!(target: "stock::update","successfully update stock");
    ApiResponse::ok(
        find_detail_stock.unwrap(),
        translate!("stock.update.success", lang).as_str(),
    )
}
