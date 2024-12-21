use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::{
    create_object_id_option, QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST,
};
use crate::dto::coach_dto::CoachDTO;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::entity::coach::Coach;
use crate::entity::file_attachment::FileAttachment;
use crate::feature::coach::coach_model::{CreateCoachRequest, UpdateCoachRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<CoachDTO>> {
    info!(target: "coach::list", "{} trying get list coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::READ) {
        info!(target: "coach::list", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "coach::list", "Branch id is null");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default.clone());
    let mut get = Orm::get("coach");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.filter_string("$text", Some("$search"), text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("full_name");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("full_name");
    }

    if filter == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if filter == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("account", "created_by_id", "_id", "created_by")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .pageable::<CoachDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    info!(target: "coach::list", "successfully get list coach");
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("coach.list.success", lang).as_str(),
    )
}

pub async fn get_detail_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(coach_id): Path<String>,
) -> ApiResponse<CoachDTO> {
    info!(target: "coach::detail", "{} trying get detail coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::READ) {
        info!(target: "coach::detail", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "coach::detail", "Branch id null");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(coach_id.as_str());
    if id.is_none() {
        info!(target: "coach::detail", "Failed create ObjectId");
        return ApiResponse::not_found(translate!("coach.not-found", lang).as_str());
    }

    let find_coach = Orm::get("coach")
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .one::<CoachDTO>(&state.db)
        .await;

    if find_coach.is_err() {
        info!(target: "coach::detail", "{}",find_coach.unwrap_err());
        return ApiResponse::not_found(translate!("coach.not-found", lang).as_str());
    }

    info!(target: "coach::detail", "Successfully get detail coach");
    ApiResponse::ok(
        find_coach.unwrap(),
        translate!("coach.found", lang).as_str(),
    )
}

pub async fn create_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<CreateCoachRequest>,
) -> ApiResponse<CoachDTO> {
    info!(target: "coach::create", "{} trying create coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::CREATE) {
        info!(target: "coach::create", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "coach::detail", "Branch id null");
        return ApiResponse::failed(translate!("coach.create.failed", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let coach = Coach {
        id: Some(ObjectId::new()),
        branch_id: auth_context.branch_id,
        created_by_id: auth_context.user_id,
        full_name: body.full_name.clone(),
        gender: body.gender.clone(),
        email: body.email.clone(),
        phone_number: body.phone_number.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let save = Orm::insert("coach").one(&coach, &state.db).await;
    if save.is_err() {
        info!(target: "coach::create", "{}",save.unwrap_err());
        return ApiResponse::failed(translate!("coach.create.failed", lang).as_str());
    }
    info!(target: "coach::create","Successfully created Coach");
    ApiResponse::ok(
        coach.to_dto(),
        translate!("coach.create.success", lang).as_str(),
    )
}

pub async fn update_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(coach_id): Path<String>,
    Json(body): Json<UpdateCoachRequest>,
) -> ApiResponse<CoachDTO> {
    info!(target: "coach::update", "{} trying update  coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::UPDATE) {
        info!(target: "coach::update", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("coach.update.failed", lang).as_str(),
        );
    }

    let coach_id = create_object_id_option(coach_id.as_str());
    if coach_id.is_none() {
        info!(target: "coach::update", "Failed create ObjectId");
        return ApiResponse::not_found(translate!("coach.not-found", lang).as_str());
    }

    let find_coach = Orm::get("coach")
        .filter_object_id("_id", &coach_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .one::<CoachDTO>(&state.db)
        .await;
    if find_coach.is_err() {
        info!(target: "coach::update", "{}",find_coach.unwrap_err());
        return ApiResponse::not_found(translate!("coach.not-found", lang).as_str());
    }
    let mut coach = find_coach.unwrap();

    let mut save = Orm::update("coach");
    if body.full_name.is_some() {
        coach.full_name = body.full_name.clone().unwrap();
        save = save.set_str("full_name", &body.full_name.clone().unwrap());
    }

    if body.email.is_some() {
        coach.email = body.email.clone().unwrap();
        save = save.set_str("email", &body.email.clone().unwrap());
    }

    if body.gender.is_some() {
        coach.gender = body.gender.clone().unwrap();
        save = save.set_str("gender", &body.gender.clone().unwrap());
    }

    if body.phone_number.is_some() {
        coach.phone_number = body.phone_number.clone().unwrap();
        if body.phone_number.is_some() {
            save = save.set_str("phone_number", &body.phone_number.clone().unwrap());
        }
    }

    let save_data = save
        .filter_object_id("_id", &coach_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .execute_one(&state.db)
        .await;

    if save_data.is_err() {
        info!(target: "coach::update", "{}",save_data.unwrap_err());

        return ApiResponse::failed(translate!("coach.update.failed", lang).as_str());
    }

    info!(target: "coach::update", "Successfully updated Coach");
    ApiResponse::ok(coach, translate!("coach.update.success", lang).as_str())
}

pub async fn delete_coach(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(coach_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "coach::delete", "{} trying delete  coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::DELETE) {
        info!(target: "coach::delete", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(coach_id.as_str());
    if id.is_none() {
        info!(target: "coach::delete", "Failed create ObjectId");
        return ApiResponse::un_authorized(translate!("coach.not-found", lang).as_str());
    }

    let update = Orm::update("coach")
        .filter_object_id("_id", &id.unwrap())
        .set_bool("deleted", true)
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        info!(target: "coach::delete","{}",update.unwrap_err());
        return ApiResponse::failed(translate!("coach.delete.failed", lang).as_str());
    }

    info!(target: "coach::delete", "Successfully deleted Coach");
    ApiResponse::ok(
        "OK".to_string(),
        translate!("coach.delete.success", lang).as_str(),
    )
}

pub async fn update_profile_picture(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    multipart: SingleFileExtractor,
) -> ApiResponse<FileAttachmentDTO> {
    info!(target: "coach::profile-picture", "{} trying update  prpfile picture coach", auth_context.claims.sub);
    if !auth_context.authorize(app::coach::UPDATE) {
        info!(target: "coach::profile-picture", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }


    let validate = multipart.validate_body();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("coach.profile-picture.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(multipart.ref_id.as_str());
    if user_id.is_none() {
        info!(target: "coach::profile-picture", "Failed create ObjectId");
        return ApiResponse::not_found(
            translate!("coach.profile-picture.not-found", lang).as_str(),
        );
    }
    let find_exist_profile_picture = Orm::get("file-attachment")
        .filter_object_id("ref_id", &user_id.unwrap())
        .one::<FileAttachment>(&state.db)
        .await;

    let multipart = multipart.file();
    let minio = MinIO::new().await;
    let mut filename = format!("{}.{}", multipart.filename, multipart.extension);
    let is_file_exists = find_exist_profile_picture.is_ok();
    let bucket_name = "coach-profile-picture".to_string();

    let attachment = match find_exist_profile_picture {
        Ok(v) => v,
        Err(_) => FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: create_object_id_option(multipart.ref_id.as_str()),
            filename: multipart.filename.clone(),
            mime_type: multipart.mime_type.clone(),
            extension: multipart.extension.clone(),
            kind: "COACH".to_string(),
            create_at: DateTime::now(),
            updated_at: DateTime::now(),
        },
    };

    if is_file_exists {
        filename = attachment.filename.clone();
        let _delete_existing = minio
            .delete_file(filename.clone(), bucket_name.clone())
            .await;
    }

    //upload new
    let minio = minio
        .upload_file(multipart.temp_path.clone(), bucket_name, filename.clone())
        .await;

    if minio.is_err() {
        let err = minio.unwrap_err();
        info!(target: "coach::profile-picture", "{}", err);
        let _remove = multipart.remove_file();
        return ApiResponse::failed(translate!("coach.profile-picture.failed", lang).as_str());
    }

    let mut error_message = String::new();
    let success = match is_file_exists {
        true => {
            let update_profile_picture = Orm::update("file-attachment")
                .filter_object_id("ref_id", &user_id.unwrap())
                .set_str("filename", &filename.as_str())
                .set_str("mime-type", &multipart.mime_type.as_str())
                .set_str("extension", &multipart.extension.as_str())
                .execute_one(&state.db)
                .await;
            if update_profile_picture.is_err() {
                error_message = update_profile_picture.clone().unwrap_err();
            }
            update_profile_picture.is_ok()
        }
        false => {
            let save_profile_picture = Orm::insert("file-attachment")
                .one(&attachment, &state.db)
                .await;
            if save_profile_picture.is_err() {
                error_message = save_profile_picture.clone().unwrap_err();
            }
            save_profile_picture.is_ok()
        }
    };

    if !success {
        info!(target: "coach::profile-picture", "{}", error_message);
        let _remove = multipart.remove_file();
        return ApiResponse::failed(translate!("coach.profile-picture.failed", lang).as_str());
    }

    let _remove = multipart.remove_file();
    info!(target: "coach::profile-picture", "Successfully update  picture coach");
    ApiResponse::ok(
        attachment.to_dto(),
        translate!("coach.profile-picture.success", lang).as_str(),
    )
}
