use crate::common::constant::{BUCKET_EVENT, BUCKET_THREAD, COLLECTION_EVENT_GUEST, EVENT_GUEST_ROLE_CO_HOST, EVENT_GUEST_ROLE_HOST, EVENT_STATUS_DRAFT, SSE_EVENT_UPDATE_EVENT_CONFIG, SSE_EVENT_UPDATE_EVENT_DATA, SSE_EVENT_UPDATE_EVENT_HOST, SSE_EVENT_UPDATE_EVENT_IMAGE, SSE_EVENT_UPDATE_EVENT_VENUE};
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::sse::sse_builder::{SseBuilder, SseTarget};
use crate::common::utils::create_object_id_option;
use crate::dto::event_guest_dto::EventGuestDTO;
use crate::dto::venue_location_dto::VenueLocationDTO;
use crate::entity::event_guest::EventGuest;
use crate::entity::event_image::EventImage;
use crate::feature::event::event_model::{
    UpdateEventConfigRequest, UpdateEventHostRequest, UpdateEventLocationRequest,
    UpdateEventRequest,
};
use crate::{
    common::{
        api_response::{ApiResponse, PaginationRequest, PagingResponse},
        app_state::AppState,
        constant::{
            COLLECTION_CONFIGURATION, COLLECTION_EVENT_IMAGES, COLLECTION_EVENT_THEMES,
            COLLECTION_EVENTS, COLLECTION_USERS,
        },
        jwt::AuthContext,
        lang::Lang,
        middleware::Json,
        mongo::{DB, filter::is, lookup::one},
    },
    dto::{event_config_dto::EventConfigDTO, event_dto::EventDTO},
    entity::{configuration::Configuration, event::Event},
    i18n,
};
use axum::extract::{Path, Query, State};
use bson::{DateTime, doc, oid::ObjectId};
use event_model::CreateNewEventRequest;
use log::info;
use serde_json::{Error, from_value};
use validator::Validate;
use crate::common::mongo::filter::is_in;
use crate::dto::mutual_dto::MutualDTO;

pub mod event_model;

pub async fn get_hosted_events(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<EventDTO>> {
    info!(target:"get_hosted_event","Starting...");
    let i18n = i18n!("event", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"get_hosted_event","user id not found or session doesn;t exist");
        return ApiResponse::failed(&i18n.translate("get_hosted_event.user_id_not_exist"));
    }
    let current_user_id = auth_context.get_user_id().unwrap();

    let mut data = DB::get(COLLECTION_EVENTS);

    if let Some(q) = query.q.clone() {
        data = data.text(q);
    }

    if let Some((col, order)) = query.clone().get_sorted() {
        data = data.sort(vec![(&col, order)]);
    }

    let find_hosted_events = data
        .filter(vec![is("host_id", current_user_id)])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_per_page::<EventDTO>(query.page.unwrap_or(0), query.size.unwrap_or(50), &state.db)
        .await;
    if let Err(why) = find_hosted_events {
        info!(target:"get_hosted_event","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("get_hosted_event.user_id_not_exist"));
    }

    ApiResponse::ok(
        find_hosted_events.unwrap(),
        &i18n.translate("get_hosted_events.ok"),
    )
}

pub async fn create_new_event(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<CreateNewEventRequest>,
) -> ApiResponse<EventDTO> {
    info!(target:"create_new_event","Starting...");
    let i18n = i18n!("event", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"create_new_event","user id not found");
        return ApiResponse::failed(&i18n.translate("create_new_event.user_id_not_exist"));
    }

    let find_default_config = DB::get(COLLECTION_CONFIGURATION)
        .filter(vec![
            is("namespace", "event-config"),
            is("name", "default-config"),
        ])
        .get_one::<Configuration>(&state.db)
        .await;

    if let Err(why) = find_default_config {
        info!(target:"create_new_event","config not found {:?}",why);
        return ApiResponse::failed(&i18n.translate("create_new_event.config_not_found"));
    }
    let find_default_config = find_default_config.unwrap();
    let default_config = find_default_config.extract_content::<EventConfigDTO>();
    if let None = default_config {
        info!(target:"create_new_event","config not found 2");
        return ApiResponse::failed(&i18n.translate("create_new_event.config_not_found"));
    }
    let default_config = default_config.unwrap();

    let create_event_id = ObjectId::new();
    let event = Event {
        id: Some(create_event_id),
        host_id: auth_context.get_user_id(),
        image_id: None,
        theme_id: None,
        invitation_id: "".to_string(),
        event_name: body.title.clone(),
        event_description: "".to_string(),
        datetime: None,
        venue_location: None,
        status: EVENT_STATUS_DRAFT.to_string(),
        config: default_config,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"create_new_event","{:?}",why);
        return ApiResponse::failed(&i18n.translate("name"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_event = DB::insert(COLLECTION_EVENTS)
        .one_with_session(event, &state.db, &mut session)
        .await;

    if let Err(why) = save_event {
        info!(target:"create_new_event","config not found {:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("create_new_event.failed"));
    }

    let _commit = session.commit_transaction().await;

    let find_hosted_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id)])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_hosted_events {
        info!(target:"get_hosted_event","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("get_hosted_event.user_id_not_exist"));
    }
    info!(target:"create_new_event","Finish");
    ApiResponse::ok(
        find_hosted_events.unwrap(),
        &i18n.translate("create_new_event.user_id_not_exist"),
    )
}

pub async fn upload_event_image(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: SingleFileExtractor,
) -> ApiResponse<EventDTO> {
    let i18n = i18n!("event", lang);
    if let Err(err) = body.validate_body() {
        info!(target:"upload_event_image","{:?}", err);
        return ApiResponse::error_validation(
            err,
            &i18n.translate("upload_event_image.validation_error"),
        );
    }
    let multipart_file = body.file();
    if let None = auth_context.get_user_id() {
        info!(target:"upload_event_image","user id not found");
        return ApiResponse::failed(&i18n.translate("upload_event_image.user_id_not_found"));
    }
    let event_id = body.ref_id_to_object();
    if let None = event_id {
        info!(target:"upload_event_image","event id invalid");
        return ApiResponse::failed(&i18n.translate("create_new_event.event_id_invalid"));
    }

    let find_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", event_id.unwrap())])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_events {
        info!(target:"upload_event_image","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("upload_event_image.event_not_found"));
    }
    let mut event = find_events.unwrap();
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"upload_event_image","{:?}",why);
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let storage = MinIO::new();
    let is_image_exist = event.image != None;
    if let Some(image) = event.image.clone() {
        let delete_existing_image = storage
            .delete_file(
                format!("{}/{}", image.path, image.file_name),
                BUCKET_EVENT.to_string(),
            )
            .await;
        if let Err(why) = delete_existing_image {
            info!(target:"upload_event_image","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(
                &i18n.translate("upload_event_image.failed_remove_existing_image"),
            );
        }
    }

    let file_name = format!("{}{}", multipart_file.ref_id, multipart_file.extension);
    let upload = storage
        .upload_file(
            multipart_file.temp_path,
            BUCKET_EVENT.to_string(),
            format!("{}/{}", multipart_file.ref_id, file_name),
        )
        .await;

    if let Err(why) = upload {
        info!(target:"upload_event_image","{:?}", why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.failed_upload"));
    }

    if is_image_exist {
        let update_image = DB::update(COLLECTION_EVENT_IMAGES)
            .set(doc! {
                "file_name":file_name.clone(),
            })
            .filter(vec![is("_id", event.image_id.clone().unwrap())])
            .execute_with_session(&state.db, &mut session)
            .await;
        if let Err(why) = update_image {
            info!(target:"upload_event_image","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("upload_event_image.failed_update_image"));
        }
        let mut image = event.image.clone().unwrap();
        image.file_name = file_name.clone();
        event.image = Some(image);
    } else {
        let image = EventImage {
            id: Some(ObjectId::new()),
            file_name: file_name.clone(),
            bucket: BUCKET_EVENT.to_string(),
            path: multipart_file.ref_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };
        let save_image = DB::insert(COLLECTION_EVENT_IMAGES)
            .one_with_session(image.clone(), &state.db, &mut session)
            .await;
        if let Err(why) = save_image {
            info!(target:"upload_event_image","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("upload_event_image.failed_save_image"));
        }

        let update_event_image = DB::update(COLLECTION_EVENTS)
            .set(doc! {
                "image_id":image.id,
            })
            .execute_with_session(&state.db, &mut session)
            .await;
        if let Err(why) = update_event_image {
            info!(target:"upload_event_image","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(
                &i18n.translate("upload_event_image.failed_update_event_image"),
            );
        }
        event.image = Some(image);
    }

    let _commit = session.commit_transaction().await;
    info!(target:"upload_event_image","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(body.ref_id.to_string())
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_IMAGE.to_string())
            .set_user_ids(find_subscriber),
        event.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"upload_event_image","finish");
    ApiResponse::ok(event, &i18n.translate("upload_event_image.ok"))
}

pub async fn update_event_data(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
    Json(body): Json<UpdateEventRequest>,
) -> ApiResponse<EventDTO> {
    let i18n = i18n!("event", lang);
    if let Err(err) = body.validate() {
        info!(target:"upload_event_image","{:?}", err);
        return ApiResponse::error_validation(
            err,
            &i18n.translate("upload_event_image.validation_error"),
        );
    }

    if let None = auth_context.get_user_id() {
        info!(target:"upload_event_image","user id not found");
        return ApiResponse::failed(&i18n.translate("upload_event_image.user_id_not_found"));
    }
    let create_event_id = create_object_id_option(&event_id);
    if let None = create_event_id {
        info!(target:"upload_event_image","event id invalid");
        return ApiResponse::failed(&i18n.translate("create_new_event.event_id_invalid"));
    }

    let find_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_events {
        info!(target:"upload_event_image","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("upload_event_image.event_not_found"));
    }
    let mut event = find_events.unwrap();
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"upload_event_image","{:?}",why);
        return ApiResponse::failed(&i18n.translate("upload_event_image.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;
    //todo:here
    let mut update =
        DB::update(COLLECTION_EVENTS).filter(vec![is("_id", create_event_id.unwrap())]);

    if let Some(event_name) = body.event_name {
        event.event_name = event_name.clone();
        update = update.set_value("event_name", event_name);
    }
    if let Some(event_description) = body.event_description {
        event.event_description = event_description.clone();
        update = update.set_value("event_description", event_description);
    }
    if let Some(status) = body.status {
        event.status = status.clone();
        update = update.set_value("status", status);
    }
    if let Some(theme_id) = body.theme_id {
        event.theme_id = Some(theme_id.clone());
        update = update.set_value("theme_id", theme_id);
    }

    let update_event = update.execute_with_session(&state.db, &mut session).await;
    if let Err(why) = update_event {
        info!(target:"upload_event_image","{:?}", why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("upload_event_image.failed"));
    }
    //end
    let _commit = session.commit_transaction().await;
    info!(target:"upload_event_image","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(event_id)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_DATA.to_string())
            .set_user_ids(find_subscriber),
        event.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"upload_event_image","finish");
    ApiResponse::ok(event, &i18n.translate("upload_event_image.ok"))
}

pub async fn update_event_config(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
    Json(body): Json<UpdateEventConfigRequest>,
) -> ApiResponse<EventDTO> {
    let i18n = i18n!("event", lang);
    if let Err(err) = body.validate() {
        info!(target:"update_event_config","{:?}", err);
        return ApiResponse::error_validation(
            err,
            &i18n.translate("update_event_config.validation_error"),
        );
    }

    if let None = auth_context.get_user_id() {
        info!(target:"update_event_config","user id not found");
        return ApiResponse::failed(&i18n.translate("update_event_config.user_id_not_found"));
    }
    let create_event_id = create_object_id_option(&event_id);
    if let None = create_event_id {
        info!(target:"update_event_config","event id invalid");
        return ApiResponse::failed(&i18n.translate("update_event_config.event_id_invalid"));
    }

    let find_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_events {
        info!(target:"update_event_config","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("update_event_config.event_not_found"));
    }
    let mut event = find_events.unwrap();
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"update_event_config","{:?}",why);
        return ApiResponse::failed(&i18n.translate("update_event_config.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;
    //todo:here
    let mut update =
        DB::update(COLLECTION_EVENTS).filter(vec![is("_id", create_event_id.unwrap())]);

    if let Some(cost) = body.cost {
        event.config.cost = cost;
        update = update.set_value("cost", cost);
    }
    if let Some(cost_type) = body.cost_type {
        event.config.cost_type = cost_type.clone();
        update = update.set_value("cost_type", cost_type);
    }
    if let Some(cost_currency) = body.cost_currency {
        event.config.cost_currency = cost_currency.clone();
        update = update.set_value("cost_currency", cost_currency);
    }
    if let Some(cost_suggested_amount) = body.cost_suggested_amount {
        event.config.cost_suggested_amount = cost_suggested_amount.clone();
        update = update.set_value("cost_suggested_amount", cost_suggested_amount);
    }
    if let Some(approval_required) = body.approval_required {
        event.config.approval_required = approval_required.clone();
        update = update.set_value("approval_required", approval_required);
    }
    if let Some(auto_reminder) = body.auto_reminder {
        event.config.auto_reminder = auto_reminder.clone();
        update = update.set_value("auto_reminder", auto_reminder);
    }
    if let Some(show_guest_name) = body.show_guest_name {
        event.config.show_guest_name = show_guest_name.clone();
        update = update.set_value("show_guest_name", show_guest_name);
    }
    if let Some(show_guest_count) = body.show_guest_count {
        event.config.show_guest_count = show_guest_count.clone();
        update = update.set_value("show_guest_count", show_guest_count);
    }
    if let Some(show_timestamp_activity) = body.show_timestamp_activity {
        event.config.show_timestamp_activity = show_timestamp_activity.clone();
        update = update.set_value("show_timestamp_activity", show_timestamp_activity);
    }
    if let Some(allow_participant_album) = body.allow_participant_album {
        event.config.allow_participant_album = allow_participant_album.clone();
        update = update.set_value("allow_participant_album", allow_participant_album);
    }
    if let Some(max_capacity) = body.max_capacity {
        event.config.max_capacity = max_capacity.clone();
        update = update.set_value("max_capacity", max_capacity);
    }
    if let Some(allow_participant_album) = body.allow_participant_album {
        event.config.allow_participant_album = allow_participant_album.clone();
        update = update.set_value("allow_participant_album", allow_participant_album);
    }
    if let Some(visibility) = body.visibility {
        event.config.visibility = visibility.clone();
        update = update.set_value("visibility", visibility);
    }

    if let Some(event_password) = body.event_password {
        if !event_password.is_empty() {
            let create_password = bcrypt::hash(&event_password, bcrypt::DEFAULT_COST);
            if let Ok(hash) = create_password {
                event.config.event_password = Some(hash);
                update = update.set_value("event_password", event_password);
            }
        }
    }

    let update_event = update.execute_with_session(&state.db, &mut session).await;
    if let Err(why) = update_event {
        info!(target:"update_event_config","{:?}", why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("update_event_config.failed"));
    }
    //todo:end
    let _commit = session.commit_transaction().await;
    info!(target:"update_event_config","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(event_id)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_CONFIG.to_string())
            .set_user_ids(find_subscriber),
        event.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"update_event_config","finish");
    ApiResponse::ok(event.clone(), &i18n.translate("update_event_config.failed"))
}

pub async fn update_event_venue(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
    Json(body): Json<UpdateEventLocationRequest>,
) -> ApiResponse<EventDTO> {
    let i18n = i18n!("event", lang);
    if let Err(err) = body.validate() {
        info!(target:"update_event_config","{:?}", err);
        return ApiResponse::error_validation(
            err,
            &i18n.translate("update_event_config.validation_error"),
        );
    }

    if let None = auth_context.get_user_id() {
        info!(target:"update_event_config","user id not found");
        return ApiResponse::failed(&i18n.translate("update_event_config.user_id_not_found"));
    }
    let create_event_id = create_object_id_option(&event_id);
    if let None = create_event_id {
        info!(target:"update_event_config","event id invalid");
        return ApiResponse::failed(&i18n.translate("update_event_config.event_id_invalid"));
    }

    let find_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_events {
        info!(target:"update_event_config","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("update_event_config.event_not_found"));
    }
    let mut event = find_events.unwrap();
    let mut location = event
        .venue_location
        .clone()
        .unwrap_or_else(|| VenueLocationDTO {
            map_id: None,
            map_details: None,
            venue_name: None,
            venue_address: None,
            venue_detail: None,
            lat: None,
            lng: None,
        });
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"update_event_config","{:?}",why);
        return ApiResponse::failed(&i18n.translate("update_event_config.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    if let Some(venue_name) = body.venue_name {
        location.venue_name = Some(venue_name.clone());
    }

    if let Some(venue_address) = body.venue_address {
        location.venue_address = Some(venue_address.clone());
    }

    if let Some(venue_detail) = body.venue_detail {
        location.venue_detail = Some(venue_detail.clone());
    }

    if let Some(lat) = body.lat {
        location.lat = Some(lat.clone());
    }
    if let Some(lng) = body.lng {
        location.lng = Some(lng.clone());
    }

    if let Some(map_id) = body.map_id {
        location.map_id = Some(map_id.clone());
    }

    if let Some(map_details) = body.map_details {
        if let Ok(map) = from_value(map_details) {
            location.map_details = Some(map.clone());
        }
    }

    let update = DB::update(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .set(doc! {
            "venue_location":location.clone()
        })
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(why) = update {
        info!(target:"update_event_config","{:?}", why);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("update_event_config.failed"));
    }
    event.venue_location = Some(location);
    let _commit = session.commit_transaction().await;
    info!(target:"update_event_config","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(event_id)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_VENUE.to_string())
            .set_user_ids(find_subscriber),
        event.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"update_event_config","finish");
    ApiResponse::ok(event.clone(), &i18n.translate("update_event_config.failed"))
}

pub async fn update_event_guest(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<UpdateEventHostRequest>,
) -> ApiResponse<EventGuestDTO> {
    let i18n = i18n!("event", lang);
    if let Err(err) = body.validate() {
        info!(target:"add_event_host","{:?}", err);
        return ApiResponse::error_validation(
            err,
            &i18n.translate("add_event_host.validation_error"),
        );
    }

    if let None = auth_context.get_user_id() {
        info!(target:"add_event_host","user id not found");
        return ApiResponse::failed(&i18n.translate("add_event_host.user_id_not_found"));
    }
    let create_event_id = create_object_id_option(&body.event_id);
    if let None = create_event_id {
        info!(target:"add_event_host","event id invalid");
        return ApiResponse::failed(&i18n.translate("add_event_host.event_id_invalid"));
    }
    let create_user_id = create_object_id_option(&body.user_id);
    if let None = create_user_id {
        info!(target:"add_event_host","event id invalid");
        return ApiResponse::failed(&i18n.translate("add_event_host.event_id_invalid"));
    }

    let find_host = DB::get(COLLECTION_EVENT_GUEST)
        .filter(vec![
            is("event_id", create_event_id.unwrap()),
            is("user_id", create_user_id.unwrap()),
        ])
        .lookup(&[
            one(COLLECTION_USERS, "user_id", "user_id", "user"),
            one(COLLECTION_EVENTS, "event_id", "event_id", "event"),
        ])
        .get_one::<EventGuestDTO>(&state.db)
        .await;

    let is_exist = find_host.is_ok();
    let host = find_host.unwrap();
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"add_event_host","{:?}",why);
        return ApiResponse::failed(&i18n.translate("add_event_host.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;
    if is_exist {
        let update = DB::update(COLLECTION_EVENT_GUEST)
            .filter(vec![is("_id", host.id.unwrap())])
            .set(doc! {
                "role":body.role.clone(),
                "updated_at":DateTime::now()
            })
            .execute_with_session(&state.db, &mut session)
            .await;
        if let Err(why) = update {
            info!(target:"add_event_host","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("add_event_host.failed"));
        }
    } else {
        let host = EventGuest {
            id: Some(ObjectId::new()),
            user_id: create_user_id,
            event_id: create_event_id,
            role: body.role.clone(),
            rsvp: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        let save = DB::insert(COLLECTION_EVENT_GUEST)
            .one_with_session(host,&state.db,&mut session)
            .await;
        if let Err(why) = save {
            info!(target:"add_event_host","{:?}", why);
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("add_event_host.failed"));
        }
    }


    let _commit = session.commit_transaction().await;
    info!(target:"add_event_host","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(body.event_id)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_HOST.to_string())
            .set_user_ids(find_subscriber),
        host.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"add_event_host","finish");
    ApiResponse::ok(host, &i18n.translate("add_event_host.failed"))
}

pub async fn delete_event(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
) -> ApiResponse<EventDTO> {
    let i18n = i18n!("event", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"delete_event","user id not found");
        return ApiResponse::failed(&i18n.translate("delete_event.user_id_not_found"));
    }

    let create_event_id = create_object_id_option(&event_id);
    if let None = create_event_id {
        info!(target:"delete_event","event id invalid");
        return ApiResponse::failed(&i18n.translate("delete_event.event_id_invalid"));
    }

    let find_events = DB::get(COLLECTION_EVENTS)
        .filter(vec![is("_id", create_event_id.unwrap())])
        .lookup(&[
            one(COLLECTION_USERS, "host_id", "_id", "host"),
            one(COLLECTION_EVENT_IMAGES, "image_id", "_id", "image"),
            one(COLLECTION_EVENT_THEMES, "theme_id", "_id", "theme"),
        ])
        .get_one::<EventDTO>(&state.db)
        .await;
    if let Err(why) = find_events {
        info!(target:"delete_event","cannot find events {}",why);
        return ApiResponse::failed(&i18n.translate("delete_event.event_not_found"));
    }
    let mut event = find_events.unwrap();
    let session = state.db.start_session().await;
    if let Err(why) = session {
        info!(target:"delete_event","{:?}",why);
        return ApiResponse::failed(&i18n.translate("delete_event.trx_failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    //todo:here
    let delete_event = DB::delete(COLLECTION_EVENTS)
        .one_with_session(&state.db, &mut session)
        .await;
    if let Err(why) = delete_event {
        info!(target:"delete_event","{:?}",why);
        return ApiResponse::failed(&i18n.translate("delete_event.trx_failed"));
    }
    //todo:end
    let _commit = session.commit_transaction().await;
    info!(target:"delete_event","sending sse");
    let find_subscriber = state
        .redis
        .get_list_subscriber(event_id)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<String>>();

    let sse = SseBuilder::new(
        SseTarget::create()
            .set_event_name(SSE_EVENT_UPDATE_EVENT_IMAGE.to_string())
            .set_user_ids(find_subscriber),
        event.clone(),
    );
    let _send = state.sse.send(sse);
    info!(target:"delete_event","finish");
    ApiResponse::ok(event, &i18n.translate("delete_event.event_not_found"))
}

//
pub async fn get_event_guest(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
    Query(query):Query<PaginationRequest>
)->ApiResponse<PagingResponse<EventGuestDTO>>{
    let i18n = i18n!("event", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"add_event_host","user id not found");
        return ApiResponse::failed(&i18n.translate("add_event_host.user_id_not_found"));
    }
    let create_event = create_object_id_option(&event_id);

    let find_host = DB::get(COLLECTION_EVENT_GUEST)
        .filter(vec![
            is("event_id", create_event.unwrap()),
        ])
        .lookup(&[
            one(COLLECTION_USERS, "user_id", "_id", "user"),
            one(COLLECTION_EVENTS, "event_id", "_id", "event"),
        ])
        .sort(vec![("created_at",-1)])
        .get_per_page::<EventGuestDTO>(
            query.page.unwrap_or(0),
            query.size.unwrap_or(50),
            &state.db
        ).await;

    if let Err(why) = find_host {
        info!(target:"add_event_host","{:?}",why);
        return ApiResponse::failed(&i18n.translate("add_event_host.event_not_found"));
    }

    ApiResponse::ok(
        find_host.unwrap(),
        &i18n.translate("find_host.event_guest"),
    )
}

pub async fn get_event_host(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(event_id): Path<String>,
    Query(query):Query<PaginationRequest>
)->ApiResponse<PagingResponse<EventGuestDTO>>{
    let i18n = i18n!("event", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"add_event_host","user id not found");
        return ApiResponse::failed(&i18n.translate("add_event_host.user_id_not_found"));
    }
    let create_event = create_object_id_option(&event_id);

    let find_host = DB::get(COLLECTION_EVENT_GUEST)
        .filter(vec![
            is("event_id", create_event.unwrap()),
            is_in("role",[EVENT_GUEST_ROLE_CO_HOST,EVENT_GUEST_ROLE_HOST])
        ])
        .lookup(&[
            one(COLLECTION_USERS, "user_id", "_id", "user"),
            one(COLLECTION_EVENTS, "event_id", "_id", "event"),
        ])
        .sort(vec![("created_at",-1)])
        .get_per_page::<EventGuestDTO>(
            query.page.unwrap_or(0),
            query.size.unwrap_or(50),
            &state.db
        ).await;

    if let Err(why) = find_host {
        info!(target:"add_event_host","{:?}",why);
        return ApiResponse::failed(&i18n.translate("add_event_host.event_not_found"));
    }

    ApiResponse::ok(
        find_host.unwrap(),
        &i18n.translate("find_host.event_guest"),
    )
}
