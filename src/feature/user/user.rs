use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::seeder::get_list_permission;
use crate::common::utils::{
    create_object_id_option, QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST,
};
use crate::dto::account_dto::{AccountDTO, AccountDetailDTO};
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::entity::account::Account;
use crate::entity::account_permission::AccountPermission;
use crate::entity::file_attachment::FileAttachment;
use crate::feature::user::user_model::{CreateUserRequest, UpdateUserRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_user(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<AccountDTO>> {
    if !auth_context.authorize(app::user::READ) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.name.clone().unwrap_or(default.clone());
    let date = query.date.clone().unwrap_or(default.clone());
    let mut get = Orm::get("account");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.text().filter_string("$search", None, text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("full_name");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("full_name");
    }

    if date == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if date == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .and()
        .filter_bool("deleted", None, false)
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .pageable::<AccountDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("user.list.success", lang).as_str(),
    )
}

pub async fn get_detail_user(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
) -> ApiResponse<AccountDetailDTO> {
    if !auth_context.authorize(app::user::READ) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(user_id.as_str());
    if id.is_none() {
        return ApiResponse::bad_request(translate!("user.id.invalid", lang).as_str());
    }

    let find_user = Orm::get("account")
        .filter_object_id("_id", &id.unwrap())
        .join_one("account", "reply_to", "_id", "report")
        .join_one("branch", "branch_id", "_id", "branch")
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
    Json(body): Json<CreateUserRequest>,
) -> ApiResponse<AccountDTO> {
    if !auth_context.authorize(app::user::CREATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(validate.unwrap_err(), translate!("", lang).as_str());
    }

    let account = Account {
        id: Some(ObjectId::new()),
        full_name: body.full_name.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
        gender: body.gender.clone(),
        job_title: body.job_title.clone(),
        report_to_id: auth_context.user_id,
        branch_id: auth_context.branch_id,
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

    ApiResponse::ok(
        account.to_dto(),
        translate!("validation.error", lang).as_str(),
    )
}

pub async fn update_user(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> ApiResponse<AccountDTO> {
    if !auth_context.authorize(app::user::UPDATE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
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
        return ApiResponse::access_denied(translate!("user.not-found", lang).as_str());
    }

    let find_user = Orm::get("account")
        .filter_object_id("_id", &user_id.unwrap())
        .one::<Account>(&state.db)
        .await;

    if find_user.is_err() {
        return ApiResponse::not_found(translate!("user.not-found", lang).as_str());
    }
    let mut user = find_user.unwrap();

    let mut save = Orm::update("account");
    if body.full_name.is_some() {
        user.full_name = body.full_name.clone().unwrap();
        save = save.set_str("full_name", &body.full_name.clone().unwrap());
    }
    if body.email.is_some() {
        user.email = body.email.clone().unwrap();
        save = save.set_str("email", &body.email.clone().unwrap());
    }
    if body.gender.is_some() {
        user.gender = body.gender.clone().unwrap();
        save = save.set_str("gender", &body.gender.clone().unwrap());
    }
    if body.job_title.is_some() {
        user.job_title = body.job_title.clone().unwrap();
        save = save.set_str("job_title", &body.job_title.clone().unwrap());
    }

    let save_data = save
        .filter_object_id("_id", &user_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
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
    if !auth_context.authorize(app::user::DELETE) {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(user_id.as_str());
    if id.is_none() {
        return ApiResponse::access_denied(translate!("user.not-found", lang).as_str());
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
    multipart: SingleFileExtractor,
) -> ApiResponse<FileAttachmentDTO> {
    if !auth_context.authorize("app::account::write") {
        return ApiResponse::access_denied(translate!("unauthorized", lang).as_str());
    }

    let validate = multipart.validate_body();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("user.profile-picture.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(multipart.ref_id.as_str());
    if user_id.is_none() {
        return ApiResponse::not_found(translate!("user.profile-picture.not-found", lang).as_str());
    }
    let find_exist_profile_picture = Orm::get("file-attachment")
        .filter_object_id("ref_id", &user_id.unwrap())
        .one::<FileAttachment>(&state.db)
        .await;

    let file = multipart.file();
    let minio = MinIO::new().await;
    let is_file_exists = find_exist_profile_picture.is_ok();
    let bucket_name = "profile-picture".to_string();

    let attachment = match find_exist_profile_picture {
        Ok(v) => v,
        Err(_) => FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: create_object_id_option(file.ref_id.as_str()),
            filename: file.filename.clone(),
            mime_type: file.mime_type.clone(),
            extension: file.extension.clone(),
            kind: "USER".to_string(),
            create_at: DateTime::now(),
            updated_at: DateTime::now(),
        },
    };

    if is_file_exists {
        let _delete_existing = minio
            .delete_file(attachment.filename.clone(), bucket_name.clone())
            .await;
    }

    //upload new
    let minio = minio
        .upload_file(file.temp_path.clone(), bucket_name, file.filename.clone())
        .await;

    if minio.is_err() {
        let err = minio.unwrap_err();
        info!(target: "upload-profile-picture", "{}", err);
        let _remove = file.remove_file();
        return ApiResponse::failed(translate!("user.profile-picture.failed", lang).as_str());
    }

    let mut error_message = String::new();
    let success: bool = match is_file_exists {
        true => {
            let update_profile_picture = Orm::update("file-attachment")
                .filter_object_id("ref_id", &user_id.unwrap())
                .set_str("filename", &file.filename.as_str())
                .set_str("mime-type", &file.mime_type.as_str())
                .set_str("extension", &file.extension.as_str())
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
        info!(target: "upload-profile-picture", "{}", error_message);
        let _remove = file.remove_file();
        return ApiResponse::failed(translate!("user.profile-picture.failed", lang).as_str());
    }

    let _remove = file.remove_file();
    ApiResponse::ok(
        attachment.to_dto(),
        translate!("user.profile-picture.success", lang).as_str(),
    )
}
