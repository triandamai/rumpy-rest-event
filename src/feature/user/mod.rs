use axum::extract::State;
use bcrypt::DEFAULT_COST;
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use log::info;
use validator::Validate;

use crate::common::constant::BUCKET_PROFILE_PICTURE;
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

    if find_user.is_err() {
        info!(target:"user::profile::failed","connection error {:?}",find_user.err());
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let data = find_user.unwrap();

    ApiResponse::ok(data, i18n.translate("user.profile").as_str())
}

pub async fn change_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<ChangePasswordRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let validate = body.validate();
    if validate.is_err() {
        let err = validate.unwrap_err();
        info!(target:"user::profile::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.profile.failed").as_str());
    }

    let user_email: Option<&String> = auth_context.session.get(REDIS_KEY_USER_EMAIL);
    if user_email.is_none() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = Orm::get("user")
        .filter_string("email", Some("$eq"), &user_email)
        .one::<User>(&state.db)
        .await;
    if find_user.is_err() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let mut user = find_user.unwrap();
    let mut meta_data = user
        .user_meta_data
        .unwrap_or(UserMetaData { providers: None });
    let mut providers = meta_data.providers.unwrap_or(Vec::new());
    //check whether user provider BASIC(email)
    if providers.contains(&"BASIC".to_string()) {
        info!(target:"change::password","user provider has BASIC, verify current password");
        let verify = bcrypt::verify(
            body.current_password,
            user.password.unwrap_or("".to_string()).as_str(),
        );
        if verify.is_err() {
            info!(target:"user::profile::failed","current password invalid");
            return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
        }
        if !verify.unwrap() {
            info!(target:"user::profile::failed","current password invalid");
            return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
        }
    } else {
        info!(target:"change::password","user provider has not BASIC, skip verify current password");
        providers.push("BASIC".to_string());
        meta_data.providers = Some(providers);
        user.user_meta_data = Some(meta_data);
    }

    let create_password = bcrypt::hash(body.new_password, DEFAULT_COST);
    if create_password.is_err() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let update_password = Orm::update("user")
        .set(doc! {
            "password":create_password.unwrap(),
            "updated_at":DateTime::now()
        })
        .filter_object_id("_id", &user.id.unwrap())
        .execute_one(&state.db)
        .await;

    if update_password.is_err() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    ApiResponse::ok(
        "OK".to_string(),
        i18n.translate("user.profile.success").as_str(),
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
    if user_email.is_none() {
        info!(target:"user::profile::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let user_email = user_email.unwrap();
    let find_user = Orm::get("user")
        .filter_string("email", Some("$eq"), &user_email)
        .one::<User>(&state.db)
        .await;

    if find_user.is_err() {
        info!(target:"user::profile::failed","user not found");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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
    if upload.is_err() {
        let _remove = form_data.remove_file();
        info!(target:"user::profile::failed","uplod file error {:?}",upload.unwrap_err());
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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

    if insert_profile_picture.is_err() {
        let _remove = form_data.remove_file();
        info!(target:"user::profile::failed","failed update data {:?}",insert_profile_picture.unwrap_err());
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    user.profile_picture = Some(profile_picture);
    info!(target:"user::profile::success","success");
    ApiResponse::ok(user, i18n.translate("user.profile.failed").as_str())
}
