use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::create_object_id_option;
use crate::dto::member_dto::MemberDTO;
use crate::dto::product_dto::ProductDTO;
use crate::dto::transaction_dto::TransactionDTO;
use crate::entity::detail_transaction::DetailTransaction;
use crate::entity::transaction::Transaction;
use crate::feature::transaction::transaction_model::{
    CreateTransactionMembershipProductRequest, CreateTransactionTopUpRequest,
};
use crate::translate;
use axum::extract::State;
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use log::info;
use validator::Validate;

pub async fn create_top_up_transaction(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateTransactionTopUpRequest>,
) -> ApiResponse<String> {
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let find_member = Orm::get("member")
        .join_one("member-subscription", "_id", "member_id", "subscription")
        .join_one("file-attachment", "ref_id", "_id", "profile_picture")
        .one::<MemberDTO>(&state.db)
        .await;

    if find_member.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    let member = find_member.unwrap();

    let transaction = Transaction {
        id: Some(ObjectId::new()),
        branch_id: auth_context.branch_id,
        member_id: member.id,
        notes: body.notes.clone().unwrap_or("".to_string()),
        total_price: body.amount,
        total_discount: body.amount,
        created_by_id: auth_context.user_id,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let detail_transaction = DetailTransaction {
        id: Some(ObjectId::new()),
        product_id: None,
        transaction_id: None,
        kind: "TOP-UP-BALANCE".to_string(),
        notes: body.notes.unwrap_or("".to_string()),
        quantity: 1,
        total: body.amount,
        total_before_discount: 0.0,
        is_membership: true,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let session = state.db.start_session().await;
    if session.is_err() {
        info!(target:"stock::update","failed to create trx session");
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_transaction = Orm::insert("transaction")
        .one_with_session(transaction, &state.db, &mut session)
        .await;
    if save_transaction.is_err() {
        info!(target:"stock::update","failed to create trx session");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    let save_detail_transaction = Orm::insert("transaction")
        .one_with_session(detail_transaction, &state.db, &mut session)
        .await;

    if save_detail_transaction.is_err() {
        info!(target:"stock::update","failed to create trx session");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    let save_detail_transaction = Orm::update("transaction")
        .inc(doc! {
          "balance":body.amount,
        })
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if save_detail_transaction.is_err() {
        info!(target:"stock::update","failed to create trx session");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    info!(target:"stock::update","failed to create trx session");
    let _ = session.commit_transaction().await;
    ApiResponse::failed(translate!("", lang).as_str())
}

pub async fn create_product_transaction(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateTransactionMembershipProductRequest>,
) -> ApiResponse<TransactionDTO> {
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if auth_context.user_id.is_none() {
        return ApiResponse::un_authorized(translate!("", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        let err = validate.unwrap_err();
        return ApiResponse::error_validation(err, translate!("", lang).as_str());
    }

    let create_member_id = create_object_id_option(body.member_id.clone().as_str());
    if create_member_id.is_none() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }
    let find_member = Orm::get("member")
        .join_one("member-subscription", "_id", "member_id", "subscription")
        .one::<MemberDTO>(&state.db)
        .await;

    if find_member.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    let member = find_member.unwrap();

    let product_id: Vec<ObjectId> = body
        .products
        .clone()
        .iter()
        .map(|v| create_object_id_option(v.product_id.as_str()))
        .collect::<Vec<Option<ObjectId>>>()
        .into_iter()
        .filter(|v| v.is_some())
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    let find_product = Orm::get("product")
        .filter_array::<ObjectId>("_id", Some("$in"), product_id)
        .join_one("file-attachment", "_id", "ref_id", "product-image")
        .all::<ProductDTO>(&state.db)
        .await;

    if find_product.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let subscription = member.subscription;
    if subscription.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    let subscription = subscription.unwrap();

    let mut balance: f64 = subscription.balance;
    let mut out_standing_balance: f64 = subscription.outstanding_balance;

    let mut total_with_balance: f64 = 0.0;
    let mut total_for_balance: f64 = 0.0;
    let mut total_discount_amount: f64 = 0.0;
    let mut detail_transaction: Vec<DetailTransaction> = Vec::new();
    let current_time = DateTime::now();
    let mut transaction = Transaction {
        id: Some(ObjectId::new()),
        branch_id: auth_context.branch_id,
        member_id: create_member_id,
        notes: body.notes.unwrap_or("".to_string()),
        total_price: total_with_balance,
        total_discount: total_discount_amount,
        created_by_id: auth_context.user_id,
        created_at: current_time,
        updated_at: current_time,
        deleted: false,
    };

    for product in find_product.unwrap() {
        if let Some(item) = product.id {
            let req = body
                .products
                .iter()
                .find(|v| v.product_id == item.to_string());
            if let Some(request) = req {
                let total_before_discount: f64 = request.quantity as f64 * product.product_price;
                let discount: f64 = total_before_discount * (request.discount / 100.0);
                let total_after_discount: f64 = total_before_discount - discount;

                if product.is_membership {
                    total_for_balance = total_for_balance + total_after_discount;
                } else {
                    total_with_balance = total_with_balance + total_after_discount;
                }
                total_discount_amount = total_discount_amount + discount;
                let detail = DetailTransaction {
                    id: Some(ObjectId::new()),
                    product_id: Some(item),
                    transaction_id: transaction.id,
                    kind: "PRODUCT".to_string(),
                    notes: request.notes.clone().unwrap_or("".to_string()),
                    quantity: request.quantity,
                    total: total_after_discount,
                    total_before_discount: total_before_discount,
                    is_membership: product.is_membership,
                    created_at: current_time,
                    updated_at: current_time,
                    deleted: false,
                };
                detail_transaction.push(detail);
            }
        }
    }

    transaction.total_price = total_with_balance + total_for_balance;
    transaction.total_discount = total_discount_amount;

    //update balance subs
    if balance < total_with_balance {
        let calculate_outstanding = total_with_balance - balance;
        balance = 0.0;
        out_standing_balance = calculate_outstanding;
    } else {
        balance = balance - total_with_balance;
    }

    let session = state.db.start_session().await;
    if session.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let session = state.db.start_session().await;
    if session.is_err() {
        info!(target:"stock::update","failed to create trx session");
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    //save transaction
    let save_transaction = Orm::insert("transaction")
        .one_with_session(&transaction, &state.db, &mut session)
        .await;
    if save_transaction.is_err() {
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let save_detail_transaction = Orm::insert("detail-transaction")
        .many_with_session(detail_transaction.clone(), &state.db, &mut session)
        .await;
    if save_detail_transaction.is_err() {
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let update_subs = Orm::update("member-subscription")
        .set_float("balance", &balance)
        .set_float("outstanding_balance", &out_standing_balance)
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if update_subs.is_err() {
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    //update subs

    let _commit = session.commit_transaction().await;
    let mut trx = transaction.to_dto();
    let detail = detail_transaction
        .iter()
        .map(|v| v.clone().to_dto())
        .collect();
    trx.details = Some(detail);

    ApiResponse::ok(trx, translate!("not yet", lang).as_str())
}
