use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::middleware::method_not_allowed;
use crate::feature;
use crate::routes;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/sync-locales", get(routes::index::generate_locales))
        .route("/test-locales", get(routes::index::test_locales))
        .route("/sse/list", get(feature::sse::sse::get_active_subscriber))
        .route("/sse/register", get(feature::sse::sse::register_sse))
        .route("/sse/send-to-user", post(feature::sse::sse::send_to_user))
        .route(
            "/sse/send-to-user-device",
            post(feature::sse::sse::send_to_user_device),
        )
        .route("/sse/broadcast", post(feature::sse::sse::send_broadcast))
        .route(
            "/sse/subscribe-topic",
            post(feature::sse::sse::subscribe_to_topic),
        )
        .route(
            "/sse/unsubscribe-topic",
            post(feature::sse::sse::unsubscribe_to_topic),
        )
        //auth
        .route("/auth/sign-in", post(feature::auth::auth::sign_in))
        .route(
            "/auth/change-password",
            post(feature::auth::auth::change_password),
        )
        .route("/auth/sign-out", post(feature::auth::auth::sign_out))
        //branch
        .route(
            "/branch/list",
            get(feature::branch::branch::get_list_branch),
        )
        .route(
            "/branch/:branch_id",
            get(feature::branch::branch::get_detail_branch),
        )
        .route("/branch", post(feature::branch::branch::create_branch))
        .route(
            "/branch/:branch_id",
            put(feature::branch::branch::update_branch),
        )
        .route(
            "/branch/:branch_id",
            delete(feature::branch::branch::delete_branch),
        )
        //user
        .route("/user/list", get(feature::user::user::get_list_user))
        .route("/user/:user_id", get(feature::user::user::get_detail_user))
        .route("/user", post(feature::user::user::create_user))
        .route("/user/:user_id", put(feature::user::user::update_user))
        .route("/user/:user_id", delete(feature::user::user::delete_user))
        .route(
            "/user/update-profile-picture",
            post(feature::user::user::upload_profile_picture),
        )
        //product
        .route(
            "/product/list",
            get(feature::product::product::get_list_product),
        )
        .route(
            "/product/:product_id",
            get(feature::product::product::get_detail_product),
        )
        .route("/product", post(feature::product::product::create_product))
        .route(
            "/product/:product_id",
            put(feature::product::product::update_product),
        )
        .route(
            "/product/:product_id",
            delete(feature::product::product::delete_product),
        )
        .route(
            "/product/update-product-image",
            post(feature::product::product::update_product_image),
        )
        //member
        .route(
            "/member/list",
            get(feature::member::member::get_list_member),
        )
        .route(
            "/member/:product_id",
            get(feature::member::member::get_detail_member),
        )
        .route(
            "/member/nfc/:product_id",
            get(feature::member::member::get_detail_member),
        )
        .route("/member", post(feature::member::member::create_member))
        .route(
            "/member/:product_id",
            put(feature::member::member::update_member),
        )
        .route(
            "/member/:product_id",
            delete(feature::member::member::delete_member),
        )
        .route(
            "/member/update-profile-picture",
            post(feature::member::member::update_profile_picture),
        )
        .route(
            "/member/transaction",
            get(feature::member::member::get_member_transaction),
        )
        .route(
            "/member/activity",
            get(feature::member::member::get_member_activity),
        )
        .route(
            "/member/update-activity",
            post(feature::member::member::upload_progress),
        )
        //coach
        .route("/coach/list", get(feature::coach::coach::get_list_coach))
        .route(
            "/coach/:coach_id",
            get(feature::coach::coach::get_detail_coach),
        )
        .route("/coach", post(feature::coach::coach::create_coach))
        .route(
            "/coach/:coach_id",
            put(feature::coach::coach::get_detail_coach),
        )
        .route(
            "/coach/:coach_id",
            delete(feature::coach::coach::delete_coach),
        )
        .route(
            "/coach/update-profile-picture",
            post(feature::coach::coach::update_profile_picture),
        )
        //discount
        .route(
            "/discount/list",
            get(feature::discount::discount::get_list_discount),
        )
        .route(
            "/discount/:discount_id",
            get(feature::discount::discount::get_detail_discount),
        )
        .route(
            "/discount",
            post(feature::discount::discount::create_discount),
        )
        .route(
            "/discount/:discount_id",
            put(feature::discount::discount::update_discount),
        )
        .route(
            "/discount/:discount_id",
            delete(feature::discount::discount::delete_discount),
        )
        //
        .route(
            "/membership/list",
            get(feature::membership::membership::get_list_membership),
        )
        .route(
            "/membership/:membership_id",
            get(feature::membership::membership::get_detail_membership),
        )
        .route(
            "/membership",
            post(feature::membership::membership::create_membership),
        )
        .route(
            "/membership/:membership_id",
            put(feature::membership::membership::update_membership),
        )
        .route(
            "/membership/:membership_id",
            delete(feature::membership::membership::delete_membership),
        )
        //stock
        .route("/stock/list", get(feature::stock::stock::get_list_stock))
        .route(
            "/stock/:product_id",
            get(feature::stock::stock::get_detail_stock),
        )
        .route("/stock", post(feature::stock::stock::update_stock))
        //transaction
        .route(
            "/transaction/top-up",
            post(feature::transaction::transaction::create_top_up_transaction),
        )
        .route(
            "/transaction/product",
            post(feature::transaction::transaction::create_product_transaction),
        )
        .layer(axum::middleware::from_fn(method_not_allowed))
        .fallback(handle_404)
        .with_state(state)
}

async fn handle_404() -> ApiResponse<String> {
    ApiResponse::not_found("Not found.")
}
