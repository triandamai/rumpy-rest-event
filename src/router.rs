use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::middleware::method_not_allowed;
use crate::{feature, routes};
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/sync-locales", get(routes::index::generate_locales))
        //SSE
        .route("/sse/list", get(feature::sse::sse::get_active_subscriber))
        .route("/sse/register", get(feature::sse::sse::register_sse))
        .route("/sse/register/public", get(feature::sse::sse::register_public_sse))
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
        //AUTH
        .route("/auth/otp", post(feature::auth::auth_otp))
        .route("/auth/verify-otp", post(feature::auth::verify_otp))
        .route("/auth/resend-otp", post(feature::auth::resend_otp))
        //USER
        .route("/user/profile", get(feature::user::get_my_profile))
        .route("/user/public-profile", get(feature::user::get_user_profile))
        .route(
            "/user/update-profile-picture",
            put(feature::user::update_profile_picture),
        )
        //EVENT
        .route("/event/hosted", get(feature::event::get_hosted_events))
        .route("/event/invited", get(feature::event::get_invited_event))
        .route("/event/get-host",get(feature::event::get_event_host))
        .route("/event/get-guest",get(feature::event::get_event_guest))
        .route("/event/create", post(feature::event::create_new_event))
        .route(
            "/event/upload-image",
            post(feature::event::upload_event_image),
        )
        .route(
            "/event/update-event-data",
            post(feature::event::update_event_data),
        )
        .route(
            "/event/update-event-config",
            post(feature::event::update_event_config),
        )
        .route(
            "/event/update-event-venue",
            post(feature::event::update_event_venue),
        )
        .route(
            "/event/update-event-guest",
            post(feature::event::update_event_guest),
        )
        .route("/event/delete", delete(feature::event::delete_event))
        .route("/event/create-invitation-link",post(feature::invitation::create_invitation_link))
        .route("/event/send-invitation",post(feature::invitation::send_invitation))
        //MUTUALS
        .route(
            "/user/{user_id}/mutual",
            get(feature::user::get_list_mutual),
        )
        //NOTIFICATION
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
