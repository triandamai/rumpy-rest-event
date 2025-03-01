use axum::extract::State;
use bcrypt::DEFAULT_COST;
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use log::info;
use validator::Validate;

use crate::common::constant::{BUCKET_PROFILE_PICTURE, PROVIDER_BASIC};
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::orm::orm::Orm;
use crate::dto::user_dto::UserDTO;
use crate::entity::user_metadata::UserMetaData;
use crate::feature::user::user_model::ChangePasswordRequest;
use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, constant::REDIS_KEY_USER_EMAIL,
        jwt::AuthContext, lang::Lang,
    },
    entity::{profile_picture::ProfilePicture, user::User},
    i18n,
};

pub mod user_model;

pub async fn get_user_profile(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
) -> ApiResponse<UserDTO> {
    let i18n = i18n!("user", lang);
    //getting connection from pool
    let user_email = auth_context
        .session
        .get(REDIS_KEY_USER_EMAIL)
        .map_or_else(|| "".to_string(), |v| v.clone());

    let find_user = Orm::get("user")
        .filter_string("email", Some("$eq"), &user_email)
        .one::<UserDTO>(&state.db)
        .await;

    if let Err(err)= find_user {
        info!(target:"user::profile::failed","connection error {:?}",err);
        return ApiResponse::not_found(i18n.translate("user.get-profile.not-found").as_str());
    }

    info!(target:"user::profile::failed","successfully get user profile");
    let data = find_user.unwrap();
    ApiResponse::ok(data, i18n.translate("user.get-profile.success").as_str())
}

pub async fn change_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<ChangePasswordRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let validate = body.validate();
    if let Err(err)= validate {
        info!(target:"user::change-password::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.change-password.validation.error").as_str());
    }

    let user_email: Option<&String> = auth_context.session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::change-password::failed","session not found {:?}",auth_context.session);
        return ApiResponse::failed(i18n.translate("user.change-password.user.not-found").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = Orm::get("user")
        .filter_string("email", Some("$eq"), &user_email)
        .one::<User>(&state.db)
        .await;
    if let Err(err)= find_user {
        info!(target:"user::change-password::failed","user not found");
        return ApiResponse::failed(i18n.translate("user.change-password.user.not-found").as_str());
    }

    let mut user = find_user.unwrap();
    let mut meta_data = user
        .user_meta_data
        .unwrap_or(UserMetaData { providers: None });
    let mut providers = meta_data.providers.unwrap_or(Vec::new());
    //check whether user provider BASIC(email)
    if providers.contains(&PROVIDER_BASIC.to_string()) {
        info!(target:"user::change-password","user provider has BASIC provider, verifying current password");
        let verify = bcrypt::verify(
            body.current_password,
            user.password.unwrap_or("".to_string()).as_str(),
        );
        if verify.is_err() {
            info!(target:"user::change-password::failed","current password invalid");
            return ApiResponse::failed(i18n.translate("user.change-password.user.invalid").as_str());
        }
        if !verify.unwrap() {
            info!(target:"user::change-password::failed","current password invalid");
            return ApiResponse::failed(i18n.translate("user.change-password.user.invalid").as_str());
        }
    } else {
        info!(target:"user::change-password","user provider has not BASIC, skip verify current password");
        providers.push(PROVIDER_BASIC.to_string());
        meta_data.providers = Some(providers);
        user.user_meta_data = Some(meta_data);
    }

    let create_password = bcrypt::hash(body.new_password, DEFAULT_COST);
    if let Err(err)= create_password {
        info!(target:"user::change-password::failed","error while create has password {:?}",err);
        return ApiResponse::failed(i18n.translate("user.change-password.user.invalid").as_str());
    }

    let update_password = Orm::update("user")
        .set(doc! {
            "password":create_password.unwrap(),
            "updated_at":DateTime::now()
        })
        .filter_object_id("_id", &user.id.unwrap())
        .execute_one(&state.db)
        .await;

    if let Err(err)= update_password {
        info!(target:"user::change-password::failed","failed to update password {:?}",err);
        return ApiResponse::failed(i18n.translate("user.change-password.user.invalid").as_str());
    }

    info!(target:"user::change-password","updated password success");
    ApiResponse::ok(
        "OK".to_string(),
        i18n.translate("user.change-password.success").as_str(),
    )
}

pub async fn update_profile_picture(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    form_data: SingleFileExtractor,
) -> ApiResponse<User> {
    let i18n = i18n!("user", lang);

    let user_email: Option<&String> = auth_context.session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::access_denied(i18n.translate("user.update-profile-picture.not-found").as_str());
    }
    let user_email = user_email.unwrap();
    let find_user = Orm::get("user")
        .filter_string("email", Some("$eq"), &user_email)
        .one::<User>(&state.db)
        .await;

    if let Err(err)= find_user {
        info!(target:"user::profile-picture::failed","user not found");
        return ApiResponse::failed(i18n.translate("user.update-profile-picture.not-found").as_str());
    }
    let mut user = find_user.unwrap();

    let upload_file = form_data.file();
    let name: String = ObjectId::new().to_string();
    let part_file_name: String = format!("{}.{}", name, upload_file.extension);
    let upload = MinIO::new()
        .upload_file(
            upload_file.temp_path,
            BUCKET_PROFILE_PICTURE.to_string(),
            part_file_name.clone(),
        )
        .await;
    if let Err(err)= upload {
        let _remove = form_data.remove_file();
        info!(target:"user::profile-picture::failed","upload file error {:?}",err);
        return ApiResponse::failed(i18n.translate("user.update-profile-picture.failed").as_str());
    }
    let _remove = form_data.remove_file();

    let profile_picture = ProfilePicture {
        id: Some(ObjectId::new()),
        mime_type: upload_file.mime_type.clone(),
        file_name: part_file_name.clone(),
        bucket_name: BUCKET_PROFILE_PICTURE.to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    let insert_profile_picture = Orm::update("user")
        .set(doc! {
            "profile_picture":bson::to_document(&profile_picture).unwrap_or(Document::new()),
            "updated_at":DateTime::now()
        })
        .filter_object_id("_id", &user.id.unwrap())
        .execute_one(&state.db)
        .await;

    if let Err(err)= insert_profile_picture {
        let _remove = form_data.remove_file();
        info!(target:"user::profile-picture::failed","failed update data {:?}",err);
        return ApiResponse::failed(i18n.translate("user.update-profile-picture.failed").as_str());
    }
    user.profile_picture = Some(profile_picture);
    info!(target:"user::profile-picture::success","success");
    ApiResponse::ok(user, i18n.translate("user.profile.failed").as_str())
}
