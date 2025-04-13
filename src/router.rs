use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::middleware::method_not_allowed;
use crate::{feature, routes};
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/sync-locales", get(routes::index::generate_locales))
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
        .route("/auth/sign-up-email", post(feature::auth::sign_up_email))
        .route(
            "/auth/sign-up-email/verify/{code}",
            post(feature::auth::sign_up_email_confirmation),
        )
        .route("/auth/sign-in-email", post(feature::auth::sign_in_email))
        .route(
            "/auth/reset-password/request",
            post(feature::auth::request_reset_password),
        )
        .route(
            "/auth/reset-password/verify/{code}",
            post(feature::auth::verify_reset_password),
        )
        .route(
            "/auth/reset-password/set",
            post(feature::auth::set_new_password),
        )
        .route("/user/profile", get(feature::user::get_my_profile))
        .route("/user/public-profile", get(feature::user::get_user_profile))
        .route(
            "/user/{user_id}/follower",
            get(feature::user::get_list_follower),
        )
        .route(
            "/user/{user_id}/following",
            get(feature::user::get_list_following),
        )
        .route("/user/{user_id}/follow", get(feature::user::follow_user))
        .route(
            "/user/{user_id}/unfollow",
            get(feature::user::unfollow_user),
        )
        .route(
            "/user/update-profile-picture",
            put(feature::user::update_profile_picture),
        )
        .route(
            "/user/change-password",
            post(feature::user::change_password),
        )
        .route(
            "/thread/public/list",
            get(feature::thread::get_list_public_thread),
        )
        .route(
            "/thread/discussion/list",
            get(feature::thread::get_list_discussion_thread),
        )
        .route(
            "/thread/{thread_id}",
            get(feature::thread::get_detail_thread),
        )
        .route(
            "/thread/comments/list/{thread_id}",
            get(feature::thread::get_list_comment_thread),
        )
        .route(
            "/thread/user/{user_id}",
            get(feature::thread::get_list_user_thread),
        )
        .route("/thread/create", post(feature::thread::create_thread))
        .route(
            "/thread/upload-attachment",
            post(feature::thread::upload_attachment),
        )
        .route(
            "/thread/update/{thread_id}",
            put(feature::thread::update_thread),
        )
        .route(
            "/thread/delete/{thread_id}",
            delete(feature::thread::delete_thread),
        )
        .route(
            "/thread/vote/down/{thread_id}",
            post(feature::thread::down_vote),
        )
        .route("/thread/vote/up/{thread_id}", post(feature::thread::upvote))
        .route(
            "/notification/me",
            get(feature::notification::get_notification_list),
        )
        .route(
            "/notification/detail",
            get(feature::notification::get_detail_notification),
        )
        .route(
            "/notification/status",
            put(feature::notification::set_read_status_notification),
        )
        .route(
            "/notification/delete",
            delete(feature::notification::delete_notification),
        )
        .layer(axum::middleware::from_fn(method_not_allowed))
        .fallback(handle_404)
        .with_state(state)
}

async fn handle_404() -> ApiResponse<String> {
    ApiResponse::not_found("Not found.")
}
