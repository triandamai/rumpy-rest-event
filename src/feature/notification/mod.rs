use std::vec;

use axum::extract::{Path, Query, State};
use log::info;

use crate::{
    common::{
        api_response::{ApiResponse, PaginationRequest, PagingResponse},
        app_state::AppState,
        jwt::AuthContext,
        lang::Lang,
        mongo::{DB, filter::is, lookup::one},
        utils::create_object_id_option,
    },
    dto::notification_log_dto::NotificationLogDTO,
    i18n,
};

pub mod notification_model;

pub async fn get_notification_list(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<NotificationLogDTO>> {
    info!(target:"get-list-notif","trying to get notification");
    let i18n = i18n!("notification", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"get-list-notif","user id not found");
        return ApiResponse::not_found("");
    }

    let user_id = auth_context.get_user_id().clone().unwrap();

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);
    let find_all_notification = DB::get("notification-log")
        .lookup(&[
            one("user", "user_id", "_id", "user"),
            one("notification", "notification_id", "_id", "notification"),
        ])
        .filter(vec![is("user_id", user_id)])
        .sort(vec![("created_at", -1)])
        .get_per_page::<NotificationLogDTO>(page, size, &state.db)
        .await;

    if let Err(why) = find_all_notification {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }
    ApiResponse::ok(find_all_notification.unwrap(), &i18n.translate(""))
}

pub async fn get_detail_notification(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(notification_id): Path<String>,
) -> ApiResponse<NotificationLogDTO> {
    let i18n = i18n!("", lang);
    if let None = auth_context.get_user_id() {
        info!(target:"get-list-notif","user id not found");
        return ApiResponse::not_found("");
    }

    let user_id = auth_context.get_user_id().clone().unwrap();
    let notification_id = create_object_id_option(&notification_id);

    let find_detail_notification = DB::get("notification-log")
        .lookup(&[
            one("user", "user_id", "_id", "user"),
            one("notification", "notification_id", "_id", "notification"),
        ])
        .filter(vec![
            is("user_id", user_id),
            is("notification_id", notification_id),
        ])
        .sort(vec![("created_at", -1)])
        .get_one::<NotificationLogDTO>(&state.db)
        .await;

    if let Err(why) = find_detail_notification {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }
    ApiResponse::ok(find_detail_notification.unwrap(), &i18n.translate(""))
}

pub async fn set_read_status_notification(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(notification_log_id): Path<String>,
) -> ApiResponse<NotificationLogDTO> {
    let i18n = i18n!("", lang);
    if let None = auth_context.get_user_id() {
        info!(target:"get-list-notif","user id not found");
        return ApiResponse::not_found("");
    }

    let user_id = auth_context.get_user_id().clone().unwrap();
    let notification_id = create_object_id_option(&notification_log_id);

    let find_detail_notification = DB::get("notification-log")
        .lookup(&[
            one("user", "user_id", "_id", "user"),
            one("notification", "notification_id", "_id", "notification"),
        ])
        .filter(vec![
            is("user_id", user_id),
            is("notification_id", notification_id),
        ])
        .sort(vec![("created_at", -1)])
        .get_one::<NotificationLogDTO>(&state.db)
        .await;

    if let Err(why) = find_detail_notification {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }

    let update_status = DB::update("notification-log")
        .filter(vec![is("_id", notification_id)])
        .set_value("is_read", true)
        .execute(&state.db)
        .await;

    if let Err(why) = update_status {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }
    ApiResponse::ok(find_detail_notification.unwrap(), &i18n.translate(""))
}

pub async fn delete_notification(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(notification_log_id): Path<String>,
) -> ApiResponse<NotificationLogDTO> {
    let i18n = i18n!("", lang);
    if let None = auth_context.get_user_id() {
        info!(target:"get-list-notif","user id not found");
        return ApiResponse::not_found("");
    }

    let user_id = auth_context.get_user_id().clone().unwrap();
    let notification_id = create_object_id_option(&notification_log_id);

    let find_detail_notification = DB::get("notification-log")
        .lookup(&[
            one("user", "user_id", "_id", "user"),
            one("notification", "notification_id", "_id", "notification"),
        ])
        .filter(vec![
            is("user_id", user_id),
            is("notification_id", notification_id),
        ])
        .sort(vec![("created_at", -1)])
        .get_one::<NotificationLogDTO>(&state.db)
        .await;

    if let Err(why) = find_detail_notification {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }

    let delete_notification = DB::delete("notification-log")
        .filter(vec![is("_id", notification_id)])
        .one(&state.db)
        .await;

    if let Err(why) = delete_notification {
        info!(target:"get-list-notif","error: {:?}", why);
        return ApiResponse::failed("");
    }
    ApiResponse::ok(find_detail_notification.unwrap(), &i18n.translate(""))
}
