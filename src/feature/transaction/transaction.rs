use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::constant::{TRANSACTION_PRODUCT, TRANSACTION_TO_UP};
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::create_object_id_option;
use crate::dto::member_cart::MemberCartDTO;
use crate::dto::member_dto::MemberDTO;
use crate::dto::transaction_dto::TransactionDTO;
use crate::entity::detail_transaction::DetailTransaction;
use crate::entity::member_cart::MemberCart;
use crate::entity::product::Product;
use crate::entity::transaction::Transaction;
use crate::feature::transaction::transaction_model::{
    CreateTransactionMembershipProductRequest, CreateTransactionTopUpRequest, InsertCartRequest,
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
) -> ApiResponse<TransactionDTO> {
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
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
        payment_method: "".to_string(),
        payment_method_provider: None,
        kind: TRANSACTION_TO_UP.to_string(),
    };

    let detail_transaction = DetailTransaction {
        id: Some(ObjectId::new()),
        product_id: None,
        transaction_id: None,
        kind: TRANSACTION_TO_UP.to_string(),
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
        info!(target:"stock::update","{:?}",session.unwrap_err());
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_transaction = Orm::insert("transaction")
        .one_with_session(transaction.clone(), &state.db, &mut session)
        .await;
    if save_transaction.is_err() {
        info!(target:"stock::update","failed to execute save transaction");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    let save_detail_transaction = Orm::insert("detail-transaction")
        .one_with_session(detail_transaction, &state.db, &mut session)
        .await;

    if save_detail_transaction.is_err() {
        info!(target:"stock::update","failed to execute save detail transaction");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    let update_balance_transaction = Orm::update("member-subscription")
        .inc(doc! {
          "balance":body.amount,
        })
        .filter_object_id("member_id", &member.id.unwrap())
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if update_balance_transaction.is_err() {
        info!(target:"stock::update","failed to execute save updadate balance transaction");
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }

    info!(target:"stock::update","Success update balance");
    let _ = session.commit_transaction().await;
    ApiResponse::ok(transaction.to_dto(), translate!("", lang).as_str())
}

pub async fn create_product_transaction(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateTransactionMembershipProductRequest>,
) -> ApiResponse<TransactionDTO> {
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if auth_context.user_id.is_none() {
        return ApiResponse::access_denied(translate!("", lang).as_str());
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

    let find_cart = Orm::get("member-cart")
        .and()
        .filter_object_id("member_id", &create_member_id.unwrap())
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("product", "product_id", "_id", "product")
        .all::<MemberCartDTO>(&state.db)
        .await;
    if find_cart.is_err() {
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
        payment_method: body.payment_method,
        payment_method_provider: body.payment_provide_name,
        kind: TRANSACTION_PRODUCT.to_string(),
    };

    for cart in find_cart.unwrap() {
        if let Some(item) = cart.id {
            if let Some(product) = cart.product {
                let total_before_discount: f64 = cart.quantity as f64 * product.product_price;
                let discount: f64 = total_before_discount * (cart.discount / 100.0);
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
                    kind: TRANSACTION_PRODUCT.to_string(),
                    notes: cart.notes.clone(),
                    quantity: cart.quantity,
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

//cart
pub async fn save_or_add_product_to_cart(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<InsertCartRequest>,
) -> ApiResponse<Vec<MemberCartDTO>> {
    info!(target:"transaction::save-product-to-cart", "trying to save product to cart");
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if auth_context.user_id.is_none() {
        return ApiResponse::access_denied(translate!("", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        let err = validate.unwrap_err();
        return ApiResponse::error_validation(err, translate!("", lang).as_str());
    }

    let create_member_id = create_object_id_option(body.member_id.as_str());
    let create_product_id = create_object_id_option(body.product_id.as_str());

    if create_member_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if create_product_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let find_product = Orm::get("product")
        .filter_object_id("_id", &create_product_id.unwrap())
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .one::<Product>(&state.db)
        .await;

    if find_product.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }

    let product = find_product.unwrap();
    if product.product_stock < 1 {
        return ApiResponse::failed(translate!("update.cart.stock-not-eligible", lang).as_str());
    }

    let find_cart = Orm::get("member-cart")
        .and()
        .filter_object_id("member_id", &create_member_id.unwrap())
        .filter_object_id("product_id", &create_product_id.unwrap())
        .one::<MemberCartDTO>(&state.db)
        .await;

    if find_cart.is_err() {
        let cart = find_cart.unwrap();
        let session = state.db.start_session().await;
        if session.is_err() {
            info!(target:"stock::update","failed to create trx session");
            return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
        }
        let mut session = session.unwrap();
        let _ = session.start_transaction().await;

        //update
        let update_cart = Orm::update("member-cart")
            .set(doc! {"quantity": body.quantity, "discount": body.discount,"updated_at":DateTime::now()})
            .execute_one_with_session(&state.db,&mut session)
            .await;
        if update_cart.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }

        let mut update_product = Orm::update("product");

        let diff = body.quantity - cart.quantity;
        update_product = update_product.dec(doc! {
            "product_stock": diff
        });

        let update = update_product
            .filter_object_id("_id", &cart.id.unwrap())
            .execute_one_with_session(&state.db, &mut session)
            .await;

        if update.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }
    } else {
        let find_product = Orm::get("product")
            .filter_object_id("_id", &create_product_id.unwrap())
            .one::<Product>(&state.db)
            .await;
        if find_product.is_err() {
            return ApiResponse::not_found(translate!("", lang).as_str());
        }
        let _product = find_product.unwrap();
        //insert new
        let product_cart = MemberCart {
            id: Some(ObjectId::new()),
            member_id: create_member_id,
            product_id: create_product_id,
            quantity: body.quantity,
            discount: body.discount,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            notes: body.notes.clone(),
        };

        let session = state.db.start_session().await;
        if session.is_err() {
            info!(target:"stock::update","failed to create trx session");
            return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
        }
        let mut session = session.unwrap();
        let _ = session.start_transaction().await;

        let save_cart = Orm::insert("member-cart")
            .one_with_session(&product_cart, &state.db, &mut session)
            .await;

        if save_cart.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }

        let update_stock = Orm::update("product")
            .dec(doc! {
                "product_stock":body.quantity
            })
            .execute_many_with_session(&state.db, &mut session)
            .await;

        if update_stock.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }
    }

    let find_cart = Orm::get("member-cart")
        .join_one("product", "product_id", "_id", "product")
        .join_one("member", "member_id", "_id", "member")
        .all::<MemberCart>(&state.db)
        .await
        .unwrap_or(vec![])
        .iter()
        .map(|v| v.clone().to_dto())
        .collect();

    ApiResponse::ok(find_cart, translate!("not yet", lang).as_str())
}

pub async fn update_or_remove_product_to_cart(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<InsertCartRequest>,
) -> ApiResponse<Vec<MemberCartDTO>> {
    info!(target:"transaction::save-product-to-cart", "trying to update product to cart");
    if !auth_context.authorize(app::transaction::CREATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if auth_context.user_id.is_none() {
        return ApiResponse::access_denied(translate!("", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        let err = validate.unwrap_err();
        return ApiResponse::error_validation(err, translate!("", lang).as_str());
    }

    let create_member_id = create_object_id_option(body.member_id.as_str());
    let create_product_id = create_object_id_option(body.product_id.as_str());

    if create_member_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    if create_product_id.is_none() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    let find_product = Orm::get("product")
        .filter_object_id("_id", &create_product_id.unwrap())
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .one::<Product>(&state.db)
        .await;

    if find_product.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }

    let product = find_product.unwrap();

    let find_cart = Orm::get("member-cart")
        .and()
        .filter_object_id("member_id", &create_member_id.unwrap())
        .filter_object_id("product_id", &create_product_id.unwrap())
        .one::<MemberCartDTO>(&state.db)
        .await;

    if find_cart.is_err() {
        let cart = find_cart.unwrap();
        let session = state.db.start_session().await;
        if session.is_err() {
            info!(target:"stock::update","failed to create trx session");
            return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
        }
        let mut session = session.unwrap();
        let _ = session.start_transaction().await;

        //update
        let update_cart = Orm::update("member-cart")
            .set(doc! {"quantity": body.quantity, "discount": body.discount,"updated_at":DateTime::now()})
            .execute_one_with_session(&state.db,&mut session)
            .await;
        if update_cart.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }

        let mut update_product = Orm::update("product");

        update_product = update_product.dec(doc! {
            "product_stock": body.quantity
        });

        let update = update_product
            .filter_object_id("_id", &cart.id.unwrap())
            .execute_one_with_session(&state.db, &mut session)
            .await;

        if update.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }
    } else {
        let find_product = Orm::get("product")
            .filter_object_id("_id", &create_product_id.unwrap())
            .one::<Product>(&state.db)
            .await;
        if find_product.is_err() {
            return ApiResponse::not_found(translate!("", lang).as_str());
        }
        let _product = find_product.unwrap();
        //insert new
        let product_cart = MemberCart {
            id: Some(ObjectId::new()),
            member_id: create_member_id,
            product_id: create_product_id,
            quantity: body.quantity,
            discount: body.discount,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            notes: body.notes.clone(),
        };

        let session = state.db.start_session().await;
        if session.is_err() {
            info!(target:"stock::update","failed to create trx session");
            return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
        }
        let mut session = session.unwrap();
        let _ = session.start_transaction().await;

        let save_cart = Orm::insert("member-cart")
            .one_with_session(&product_cart, &state.db, &mut session)
            .await;

        if save_cart.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }

        let update_stock = Orm::update("product")
            .dec(doc! {
                "product_stock":body.quantity
            })
            .execute_many_with_session(&state.db, &mut session)
            .await;

        if update_stock.is_err() {
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(translate!("", lang).as_str());
        }
    }

    let find_cart = Orm::get("member-cart")
        .join_one("product", "product_id", "_id", "product")
        .join_one("member", "member_id", "_id", "member")
        .all::<MemberCart>(&state.db)
        .await
        .unwrap_or(vec![])
        .iter()
        .map(|v| v.clone().to_dto())
        .collect();

    ApiResponse::ok(find_cart, translate!("not yet", lang).as_str())
}
