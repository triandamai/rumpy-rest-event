use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::feature;
use crate::routes;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        //SSE
        .route("/sse/list", get(feature::sse::sse::get_active_subscriber))
        .route("/sse/register", get(feature::sse::sse::register_sse))
        .route("/sse/send-to-user", post(feature::sse::sse::send_to_user))
        .route("/sse/send-to-user-device", post(feature::sse::sse::send_to_user_device))
        .route("/sse/broadcast", post(feature::sse::sse::send_broadcast))

        .route("/sse/subscribe-topic", post(feature::sse::sse::subscribe_to_topic))
        .route("/sse/unsubscribe-topic", post(feature::sse::sse::unsubscribe_to_topic))
        //EMAIL-CHECK
        .route(
            "/auth/check-email-exist",
            post(feature::auth::sign_in::check_email_exist),
        )
        //SIGN IN WITH EMAIL
        .route(
            "/auth/sign-in-email",
            post(feature::auth::sign_in::sign_in_email),
        )
        .route(
            "/auth/sign-in-email/verify-otp",
            post(feature::auth::sign_in::verify_otp),
        )
        .route(
            "/auth/sign-in-email/resend-otp",
            post(feature::auth::sign_in::resend_otp),
        )
        //SIGN UP WITH EMAIL
        .route(
            "/auth/sign-up-email",
            post(feature::auth::sign_up::sign_up_email),
        )
        .route(
            "/auth/sign-up-email/verify-otp",
            post(feature::auth::sign_up::verify_otp),
        )
        .route(
            "/auth/sign-up-email/resend-otp",
            post(feature::auth::sign_up::resend_otp),
        )
        .route(
            "/auth/sign-up-email/complete",
            post(feature::auth::sign_up::complete_sign_up),
        )
        //FORGOT - PASSWORD
        .route(
            "/auth/forgot-password",
            post(feature::auth::forgot_password::forgot_password),
        )
        .route(
            "/auth/forgot-password/verify-otp",
            post(feature::auth::forgot_password::verify_otp),
        )
        .route(
            "/auth/forgot-password/resend-otp",
            post(feature::auth::forgot_password::resend_otp),
        )
        .route(
            "/auth/forgot-password/complete",
            post(feature::auth::forgot_password::complete_forgot_password),
        )


        .route(
            "/user/get-list-users",
            get(feature::user::user::get_list_user_with_paging),
        )
        .route("/user/find", get(feature::user::user::get_user_by_username))
        //threads
        .route("/thread/all", get(feature::threads::threads::get_list_threads))
        .route("/thread/all/filter", post(feature::threads::threads::get_list_filter_threads))

        .route("/thread/current-user", get(feature::threads::threads::get_list_threads_by_current_user))
        .route("/thread/current-user/filter", post(feature::threads::threads::get_list_filter_threads_by_current_user))

        .route("/thread/detail/:thread_id", get(feature::threads::threads::get_detail_thread))
        .route("/thread/create", post(feature::threads::threads::create_new_thread))

        .route("/thread/send-vote", post(feature::threads::threads::send_thread_vote))
        .route("/thread/undo-vote", post(feature::threads::threads::undo_thread_vote))

        .route("/thread/comment/:thread_id", get(feature::threads::threads::get_list_comment_by_thread))
        .route("/thread/comment/reply/:comment_id", get(feature::threads::threads::get_list_reply_comment))
        .route("/thread/comment/create", post(feature::threads::threads::create_comment))

        .route("/thread/comment/send-vote", post(feature::threads::threads::send_comment_vote))
        .route("/thread/comment/undo-vote", post(feature::threads::threads::undo_comment_vote))

        .route("/thread/attachment", post(feature::file::file::upload_thread_attachment))
        .route("/thread/attachment/:thread_id", delete(feature::file::file::delete_thread_attachment))
        .route("/thread/attachment", put(feature::file::file::update_thread_attachment))
        .fallback(handle_404)
        .with_state(state)
}

async fn handle_404() -> ApiResponse<String> {
    ApiResponse::not_found("Not found.".to_string())
}