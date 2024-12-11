use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::multipart_file::MultipartFile;
use crate::common::orm::orm::Orm;
use crate::common::seeder::get_list_permission;
use crate::common::utils::{
    create_object_id_option, QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST,
};
use crate::dto::account_dto::{AccountDTO, AccountDetailDTO};
use crate::entity::account::Account;
use crate::entity::account_permission::AccountPermission;
use crate::entity::file_attachment::FileAttachment;
use crate::feature::user::user_model::{CreateUserRequest, UpdateUserRequest};
use crate::translate;
use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;
use chrono::{NaiveDate, Utc};
use log::info;
use validator::Validate;

pub async fn get_list_user(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<AccountDTO>> {
    if !auth_context.authorize("app::account::read") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default.clone());
    let mut get = Orm::get("account");

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
        .filter_bool("deleted", None, false)
        .pageable::<AccountDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("user.list.success", lang).as_str(),
    )
}

pub async fn get_detail_user(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
) -> ApiResponse<AccountDetailDTO> {
    if !auth_context.authorize("app::account::read") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(user_id.as_str());
    if id.is_none() {
        return ApiResponse::un_authorized(translate!("user.not-found", lang).as_str());
    }

    let find_user = Orm::get("account")
        .filter_object_id("_id", &id.unwrap())
        .join_one("account", "reply_to", "_id", "report")
        .join_one("branch", "_id", "branch_id", "branch")
        .join_many("account-permission", "_id", "account_id", "permission")
        .one::<AccountDetailDTO>(&state.db)
        .await;

    if find_user.is_err() {
        return ApiResponse::not_found(translate!("user.not-found", lang).as_str());
    }

    ApiResponse::ok(find_user.unwrap(), translate!("user.found", lang).as_str())
}

pub async fn create_user(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    body: Json<CreateUserRequest>,
) -> ApiResponse<AccountDTO> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(validate.unwrap_err(), translate!("", lang).as_str());
    }
    let dob = body.date_of_birth.clone().map_or_else(||None, |v| {
        NaiveDate::parse_from_str(v.as_str(), "%Y-%m-%d").map_or(None, |v| Some(v))
    });
    let account = Account {
        id: Some(ObjectId::new()),
        full_name: body.full_name.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
        gender: body.gender.clone(),
        job_title: body.job_title.clone(),
        report_to: auth_context.user_id,
        branch_id: auth_context.branch_id,
        date_of_birth: dob,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let save = Orm::insert("account").one(&account, &state.db).await;
    if save.is_err() {
        return ApiResponse::failed(translate!("user.not-found", lang).as_str());
    }
    let account_id = save.unwrap();

    let account_permission = get_list_permission()
        .iter()
        .filter(|p| {
            !p.value.starts_with("app::branch")
                && !p.value.starts_with("app::account")
                && !p.value.starts_with("app::admin")
        })
        .map(|p| AccountPermission {
            id: Some(ObjectId::new()),
            account_id: Some(account_id.clone()),
            permission_id: Some(p.id.unwrap()),
            name: p.name.clone(),
            value: p.value.clone(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            deleted: false,
        })
        .collect::<Vec<AccountPermission>>();

    let _save_permission = Orm::insert("account-permission")
        .many(account_permission, &state.db)
        .await;

    ApiResponse::ok(account.to_dto(), translate!("", lang).as_str())
}

pub async fn update_user(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(user_id): Path<String>,
    body: Json<UpdateUserRequest>,
) -> ApiResponse<AccountDTO> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("user.update.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(user_id.as_str());
    if user_id.is_none() {
        return ApiResponse::un_authorized(translate!("user.not-found", lang).as_str());
    }

    let find_user = Orm::get("account")
        .filter_object_id("_id", &user_id.unwrap())
        .one::<Account>(&state.db)
        .await;
    if find_user.is_err() {
        return ApiResponse::not_found(translate!("user.not-found", lang).as_str());
    }
    let user = find_user.unwrap();

    let mut save = Orm::update("account");
    if body.full_name.is_some() {
        save = save.set_str("full_name", &body.full_name.clone().unwrap());
    }
    if body.email.is_some() {
        save = save.set_str("email", &body.email.clone().unwrap());
    }
    if body.gender.is_some() {
        save = save.set_str("gender", &body.gender.clone().unwrap());
    }
    if body.job_title.is_some() {
        save = save.set_str("job_title", &body.job_title.clone().unwrap());
    }
    if body.date_of_birth.is_some() {
        save = save.set_str("date_of_birth", &body.date_of_birth.clone().unwrap());
    }

    let save_data = save
        .filter_object_id("_id", &user_id.unwrap())
        .execute_one(&state.db)
        .await;

    if save_data.is_err() {
        return ApiResponse::failed(translate!("user.update.failed", lang).as_str());
    }
    ApiResponse::ok(
        user.to_dto(),
        translate!("user.update.success", lang).as_str(),
    )
}

pub async fn delete_user(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(user_id): Path<String>,
) -> ApiResponse<String> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(user_id.as_str());
    if id.is_none() {
        return ApiResponse::un_authorized(translate!("user.not-found", lang).as_str());
    }

    let update = Orm::update("account")
        .filter_object_id("_id", &id.unwrap())
        .set_bool("deleted", true)
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        return ApiResponse::failed(translate!("user.delete.failed", lang).as_str());
    }

    ApiResponse::ok(
        "OK".to_string(),
        translate!("user.delete.success", lang).as_str(),
    )
}

pub async fn upload_profile_picture(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    multipart: Multipart,
) -> ApiResponse<FileAttachment> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let extract = MultipartFile::extract_multipart(multipart).await;

    let validate = extract.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("user.profile-picture.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(extract.ref_id.as_str());
    if user_id.is_none() {
        return ApiResponse::not_found(translate!("user.profile-picture.not-found", lang).as_str());
    }
    let find_exist_profile_picture = Orm::get("file-attachment")
        .filter_object_id("ref_id", &user_id.unwrap())
        .one::<FileAttachment>(&state.db)
        .await;

    let minio = MinIO::new().await;
    let mut filename = format!("{}.{}", extract.filename, extract.extension);
    let mut is_file_exists = find_exist_profile_picture.is_ok();

    let attachment = match find_exist_profile_picture {
        Ok(v) => v,
        Err(_) => FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: create_object_id_option(extract.ref_id.as_str()),
            filename: extract.filename.clone(),
            mime_type: extract.mime_type.clone(),
            extension: extract.extension.clone(),
            kind: "USER".to_string(),
            create_at: DateTime::now(),
            updated_at: DateTime::now(),
        },
    };

    if is_file_exists {
        filename = attachment.filename.clone();
        let _delete_existing = minio
            .delete_file(filename.clone(), "profile-picture".to_string())
            .await;
    }

    //upload new
    let minio = minio
        .upload_file(
            extract.temp_path.clone(),
            "profile-picture".to_string(),
            filename.clone(),
        )
        .await;

    if minio.is_err() {
        let err = minio.unwrap_err();
        info!(target: "upload-profile-picture", "{}", err);
        let _remove = extract.remove_file();
        return ApiResponse::failed(translate!("user.profile-picture.failed", lang).as_str());
    }

    let mut error_message = String::new();
    let mut success = false;
    if is_file_exists {
        let update_profile_picture = Orm::update("file-attachment")
            .filter_object_id("ref_id", &user_id.unwrap())
            .set_str("filename", &filename.as_str())
            .set_str("mime-type", &extract.mime_type.as_str())
            .set_str("extension", &extract.extension.as_str())
            .execute_one(&state.db)
            .await;
        if update_profile_picture.is_err() {
            error_message = update_profile_picture.clone().unwrap_err();
        }
        success = update_profile_picture.is_ok();
    } else {
        let save_profile_picture = Orm::insert("file-attachment")
            .one(&attachment, &state.db)
            .await;
        if save_profile_picture.is_err() {
            error_message = save_profile_picture.clone().unwrap_err();
        }
        success = save_profile_picture.is_ok();
    }

    if !success {
        info!(target: "upload-profile-picture", "{}", error_message);
        let _remove = extract.remove_file();
        return ApiResponse::failed(translate!("user.profile-picture.failed", lang).as_str());
    }

    let _remove = extract.remove_file();
    ApiResponse::ok(
        attachment,
        translate!("user.profile-picture.success", lang).as_str(),
    )
}
