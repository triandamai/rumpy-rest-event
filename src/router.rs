use crate::common::api_response::ApiResponse;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;

use crate::common::app_state::AppState;
use crate::common::jwt::JwtUtil;
use crate::feature;
use crate::feature::conversation::conversation_model::GenerateToken;
use crate::routes;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/token", get(generate_token))

        .route("/sse/list", get(feature::sse::sse::get_active_subscriber))
        .route("/sse/register", get(feature::sse::sse::register_sse))
        .route("/sse/send-to-user", post(feature::sse::sse::send_to_user))
        .route(
            "/sse/send-to-user-device",
            post(feature::sse::sse::send_to_user_device),
        )
        .route("/sse/broadcast", post(feature::sse::sse::send_broadcast))
        .route(
            "/auth/check-email",
            post(feature::auth::sign_in::check_email),
        )
        .route(
            "/auth/sign-in-email",
            post(feature::auth::sign_in::sign_in_email),
        )
        .route(
            "/auth/sign-in-email/verify-otp",
            post(feature::auth::sign_in::verify_otp_sign_in_email),
        )
        .route(
            "/auth/sign-in-email/resend-otp",
            post(feature::auth::sign_in::resend_otp_sign_in_email),
        )
        .route(
            "/auth/sign-up-email",
            post(feature::auth::sign_up::sign_up_email),
        )
        .route(
            "/auth/sign-up-email/verify-otp",
            post(feature::auth::sign_up::verify_otp_sign_up_email),
        )
        .route(
            "/auth/sign-up-email/resend-otp",
            post(feature::auth::sign_up::resend_otp_sign_up_email),
        )
        .route(
            "/conversation/list",
            get(feature::conversation::conversation::get_conversations),
        )
        .route(
            "/conversation/direct/create",
            post(feature::conversation::conversation::create_direct_conversation),
        )
        .route(
            "/conversation/message/send-text",
            post(feature::conversation::conversation::send_text_message),
        )
        .route(
            "/conversation/message/list/:id_conversation",
            get(feature::conversation::conversation::get_messages),
        )
        .route(
            "/user/get-list-users",
            get(feature::user::user::get_list_user_with_paging),
        )
        .route(
            "/user/find",
            get(feature::user::user::get_user_by_username),
        )

        .route(
            "/friend/get-list-friends",
            get(feature::friend::friend::get_list_friend),
        )
        .route(
            "/friend/send-friend-request",
            post(feature::friend::friend::send_friend_request),
        )
        .route(
            "/friend/cancel-friend-request",
            post(feature::friend::friend::send_friend_request),
        )
        .route(
            "/friend/accept-friend-request",
            post(feature::friend::friend::accept_friend_request),
        )
        .route(
            "/friend/reject-friend-request",
            post(feature::friend::friend::reject_friend_request),
        )

        .route(
            "/space/get-list-space",
            get(feature::space::space::get_list_space_with_pagination),
        )
        .route(
            "/space/get-following-space",
            get(feature::space::space::get_list_following_space_current_user),
        )
        .route(
            "/space/detail/:space_id",
            get(feature::space::space::get_detail_space),
        )
        .route(
            "/space/create-space",
            post(feature::space::space::create_space),
        )
        .route(
            "/space/delete-space/:space_id",
            delete(feature::space::space::delete_space),
        )
        .route(
            "/space/follow-space/:space_id",
            post(feature::space::space::follow_space),
        )

        .route(
            "/post/get-list-post",
            get(feature::post::post::get_list_post_with_paging),
        )
        .route(
            "/post/get-list-post-by-current-user",
            get(feature::post::post::get_list_post_by_current_user),
        )
        .route(
            "/post/get-list-post-by-space/:space_id",
            get(feature::post::post::get_list_post_by_space),
        )
        .route(
            "/post/detail/:post_id",
            get(feature::post::post::get_detail_post),
        )
        .route(
            "/post/get-list-comments/:post_id",
            get(feature::post::post::get_list_comment_by_post),
        )
        .route(
            "/post/create-thought/:post_id",
            post(feature::post::post::create_post),
        )
        .route(
            "/post/delete-thought/:post_id",
            delete(feature::post::post::delete_post),
        )
        .with_state(state)
}

async fn generate_token(query: Query<GenerateToken>) -> impl IntoResponse {
    let token = JwtUtil::encode(query.email.clone());

    ApiResponse::ok(token.unwrap(), "sas")
}
