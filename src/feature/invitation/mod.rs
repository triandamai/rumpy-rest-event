use std::collections::HashMap;
use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::constant::{COLLECTION_EVENT_GUEST, COLLECTION_EVENT_IMAGES, COLLECTION_EVENT_INVITATION, COLLECTION_EVENTS, COLLECTION_USERS, EVENT_GUEST_ROLE_CO_HOST, EVENT_GUEST_ROLE_HOST, INVITATION_TYPE_PUBLIC, INVITATION_TYPE_USER, NOTIFICATION_TYPE_INVITATION, REDIS_KEY_USER_DISPLAY_NAME, REDIS_KEY_USER_ID, COLLECTION_NOTIFICATION, COLLECTION_NOTIFICATION_LOG};
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::mongo::DB;
use crate::common::mongo::filter::{is, is_in};
use crate::common::mongo::lookup::one;
use crate::common::utils::{
    create_object_id_option, generate_member_code, string_to_bson_datetime,
};
use crate::dto::event_dto::EventDTO;
use crate::dto::event_guest_dto::EventGuestDTO;
use crate::dto::event_invitation_dto::EventInvitationDTO;
use crate::entity::event::Event;
use crate::entity::event_invitation::EventInvitation;
use crate::entity::notification::Notification;
use crate::entity::notification_log::NotificationLog;
use crate::feature::invitation::invitation_model::{
    CreateInvitationLinkRequest, SendInvitationRequest,
};
use crate::i18n;
use axum::extract::{Path, State};
use bson::DateTime;
use bson::oid::ObjectId;
use log::info;
use std::fmt::format;

mod invitation_model;

pub async fn create_invitation_link(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<CreateInvitationLinkRequest>,
) -> ApiResponse<EventInvitationDTO> {
    info!(target:"create_invitation_link","start");
    let i18n = i18n!("invitation", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"create_invitation_link","user id not found");
        return ApiResponse::access_denied(
            &i18n.translate("create_invitation_link.user_id_invalid"),
        );
    }

    let create_event_id = create_object_id_option(&body.event_id);
    if let None = create_event_id {
        info!(target:"create_invitation_link","create_event_id none");
        return ApiResponse::not_found(&i18n.translate("create_invitation_link.event_id_invalid"));
    }
    let find_event = DB::get(COLLECTION_EVENT_GUEST)
        .filter(vec![
            is("event_id", create_event_id.unwrap()),
            is("user_id", auth_context.get_user_id().unwrap()),
            is_in("role", [EVENT_GUEST_ROLE_HOST, EVENT_GUEST_ROLE_CO_HOST]),
        ])
        .lookup(&[
            one(COLLECTION_EVENTS, "event_id", "_id", "event"),
            one(COLLECTION_USERS, "user_id", "_id", "user"),
        ])
        .get_one::<EventGuestDTO>(&state.db)
        .await;
    if let Err(err) = find_event {
        info!(target:"create_invitation_link","Error finding event host or user is guest: {:?}",err);
        return ApiResponse::not_found(&i18n.translate("create_invitation_link.invalid_role"));
    }

    let expired_at =
        string_to_bson_datetime(body.expired_at.clone()).map_or_else(|_| None, |value| Some(value));

    let invitation = EventInvitation {
        id: Some(ObjectId::new()),
        user_id: None,
        event_id: create_event_id,
        invitation_type: INVITATION_TYPE_PUBLIC.to_string(),
        invitation_code: ObjectId::new().to_string(),
        expires_at: expired_at,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let save_invitation = DB::insert(COLLECTION_EVENT_INVITATION)
        .one(invitation.clone(), &state.db)
        .await;

    if let Err(err) = save_invitation {
        info!(target:"create_invitation_link","Error saving invitation: {:?}", err);
        return ApiResponse::failed(&i18n.translate("create_invitation_link.invalid_role"));
    }

    info!(target:"create_invitation_link","finish");
    ApiResponse::ok(
        invitation.into(),
        &i18n.translate("create_invitation_link.invalid_role"),
    )
}

pub async fn send_invitation(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<SendInvitationRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("invitation", lang);
    if let None = auth_context.get_user_id() {
        info!(target:"create_invitation_link","user id not found");
        return ApiResponse::access_denied(
            &i18n.translate("create_invitation_link.user_id_invalid"),
        );
    }

    let create_event_id = create_object_id_option(&body.event_id);
    if let None = create_event_id {
        info!(target:"create_invitation_link","create_event_id none");
        return ApiResponse::not_found(&i18n.translate("create_invitation_link.event_id_invalid"));
    }

    let find_event = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .get_one::<EventDTO>(&state.db)
        .await;

    if let Err(why)=find_event {
        info!(target:"create_invitation_link","Error finding event from user: {:?}", why);
        return ApiResponse::not_found(&i18n.translate("create_invitation_link.not found"));
    }
    let event= find_event.unwrap();

    let default_display_name = i18n.translate("default_display_name");
    let mut args = HashMap::new();
    args.insert("event_name".to_string(),event.event_name.clone());
    let notification_message = i18n.translate_with_args("notification_message",args);

    let find_user = auth_context
        .get(REDIS_KEY_USER_DISPLAY_NAME)
        .unwrap_or(&default_display_name);

    let build_invitation = body
        .users
        .into_iter()
        .map(|user| {
            let date = string_to_bson_datetime(user.expired_at.clone())
                .map_or_else(|_| None, |value| Some(value));

            let create_user_id = create_object_id_option(&user.user_id);
            let create_event_id = create_object_id_option(&body.event_id);
            let create_invitation_id = ObjectId::new();

            let notification = Notification {
                id: Some(ObjectId::new()),
                ref_id: Some(create_invitation_id.clone()),
                kind: NOTIFICATION_TYPE_INVITATION.to_string(),
                title: format!("{} {}", find_user.clone(), notification_message),
                body: None,
                notification_type: "".to_string(),
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            };

            let log = NotificationLog {
                id: Some(ObjectId::new()),
                notification_id: notification.id.clone(),
                user_id: create_user_id.clone(),
                is_read: false,
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            };

            let invitation = EventInvitation {
                id: Some(create_invitation_id),
                user_id: create_user_id,
                event_id: create_event_id,
                invitation_type: INVITATION_TYPE_USER.to_string(),
                invitation_code: ObjectId::new().to_string(),
                expires_at: date,
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            };
            (notification, log, invitation)
        })
        .collect::<Vec<(Notification, NotificationLog, EventInvitation)>>();

    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"upload_event_image","{:?}",why);
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_invitation = DB::insert(COLLECTION_EVENT_INVITATION)
        .many_with_session(build_invitation.into_iter().map(|(_,_,invitation)|invitation).collect(), &state.db, &mut session)
        .await;

    if let Err(why) = save_invitation {
        info!(target:"upload_event_image","{:?}",why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }

    let save_invitation = DB::insert(COLLECTION_EVENT_INVITATION)
        .many_with_session(build_invitation.clone().into_iter().map(|(_,_,invitation)|invitation).collect(), &state.db, &mut session)
        .await;

    if let Err(why) = save_invitation {
        info!(target:"upload_event_image","{:?}",why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }

    let save_notification = DB::insert(COLLECTION_NOTIFICATION)
        .many_with_session(build_invitation.clone().into_iter().map(|(notification,_,_)|notification).collect(), &state.db, &mut session)
        .await;

    if let Err(why) = save_notification {
        info!(target:"upload_event_image","{:?}",why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }

    let save_notification_log = DB::insert(COLLECTION_NOTIFICATION_LOG)
        .many_with_session(build_invitation.clone().into_iter().map(|(_,log,_)|log).collect(), &state.db, &mut session)
        .await;

    if let Err(why) = save_notification_log {
        info!(target:"upload_event_image","{:?}",why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }
    let _commit = session.commit_transaction().await;
    //todo: send push

    //todo: end push

    ApiResponse::ok(
        "OK".to_string(),
        &i18n.translate("upload_event_image.ok"),
    )
}