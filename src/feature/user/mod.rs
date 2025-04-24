use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::{DateTime, Document, doc};
use log::info;

use crate::common::api_response::{PaginationRequest, PagingResponse};
use crate::common::constant::{
    BUCKET_PROFILE_PICTURE, COLLECTION_MUTUALS, COLLECTION_USERS, PATH_PROFILE_PICTURE,
};
use crate::common::minio::MinIO;
use crate::common::mongo::DB;
use crate::common::mongo::filter::{equal, is};
use crate::common::mongo::lookup::one;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::utils::create_object_id_option;
use crate::dto::following_dto::FollowingDTO;
use crate::dto::profile_picture_dto::ProfilePictureDTO;

use crate::dto::user_dto::UserDTO;
use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, constant::REDIS_KEY_USER_EMAIL,
        jwt::AuthContext, lang::Lang,
    },
    i18n,
};

pub mod user_model;

pub async fn get_my_profile(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
) -> ApiResponse<UserDTO> {
    let i18n = i18n!("user", lang);
    //getting connection from pool
    let user_email = auth_context
        .get(REDIS_KEY_USER_EMAIL)
        .map_or_else(|| "".to_string(), |v| v.clone());

    let find_user = DB::get(COLLECTION_USERS)
        .filter(vec![equal("email", &user_email)])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Err(err) = find_user {
        info!(target:"user::profile::failed","connection error {:?}",err);
        return ApiResponse::not_found(i18n.translate("user.get-profile.not-found").as_str());
    }

    info!(target:"user::profile::failed","successfully get user profile");
    let data = find_user.unwrap();
    ApiResponse::ok(data, i18n.translate("user.get-profile.success").as_str())
}

pub async fn update_profile_picture(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    form_data: SingleFileExtractor,
) -> ApiResponse<UserDTO> {
    let i18n = i18n!("user", lang);

    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::access_denied(
            i18n.translate("user.update-profile-picture.not-found")
                .as_str(),
        );
    }
    let user_email = user_email.unwrap();
    let find_user = DB::get(COLLECTION_USERS)
        .filter(vec![equal("email", &user_email)])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Err(err) = find_user {
        info!(target:"user::profile-picture::failed","user not found {:?}",err);
        return ApiResponse::failed(
            i18n.translate("user.update-profile-picture.not-found")
                .as_str(),
        );
    }
    let mut user = find_user.unwrap();

    let upload_file = form_data.file();
    let name: String = ObjectId::new().to_string();
    let part_file_name: String = format!("{}.{}", name, upload_file.extension);
    let minio = MinIO::new();

    let upload = minio
        .upload_file(
            upload_file.temp_path,
            BUCKET_PROFILE_PICTURE.to_string(),
            part_file_name.clone(),
        )
        .await;
    if let Err(err) = upload {
        let _remove = form_data.remove_file();
        info!(target:"user::profile-picture::failed","upload file error {:?}",err);
        return ApiResponse::failed(
            i18n.translate("user.update-profile-picture.failed")
                .as_str(),
        );
    }
    let _remove = form_data.remove_file();

    //delete existing image
    if let Some(existing) = user.profile_picture {
        let _delete = minio
            .delete_file(
                format!("{}/{}", existing.path, existing.file_name),
                existing.bucket,
            )
            .await;
    }

    //

    let profile_picture = ProfilePictureDTO {
        mime_type: upload_file.mime_type.clone(),
        path: PATH_PROFILE_PICTURE.to_string(),
        file_name: part_file_name.clone(),
        bucket: BUCKET_PROFILE_PICTURE.to_string(),
    };
    let insert_profile_picture = DB::update(COLLECTION_USERS)
        .set(doc! {
            "profile_picture":bson::to_document(&profile_picture).unwrap_or(Document::new()),
            "updated_at":DateTime::now()
        })
        .filter(vec![is("_id", &user.id)])
        .execute(&state.db)
        .await;

    if let Err(err) = insert_profile_picture {
        let _remove = form_data.remove_file();
        info!(target:"user::profile-picture::failed","failed update data {:?}",err);
        return ApiResponse::failed(
            i18n.translate("user.update-profile-picture.failed")
                .as_str(),
        );
    }

    user.profile_picture = Some(profile_picture);
    info!(target:"user::profile-picture::success","success");
    ApiResponse::ok(user, i18n.translate("user.profile.failed").as_str())
}

pub async fn get_user_profile(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Query(user_id): Query<String>,
) -> ApiResponse<UserDTO> {
    let i18n = i18n!("user", lang);
    if let None = auth_context.get_user_id() {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    let create_user_id = create_object_id_option(&user_id);
    if let None = create_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    let data = DB::get(COLLECTION_USERS)
        .lookup(&[one("user-profile", "_id", "_id", "profile")])
        .filter(vec![is("_id", create_user_id.unwrap())])
        .get_one::<UserDTO>(&state.db)
        .await;
    if let Err(why) = data {
        info!(target:"user::profile-picture::failed","{}",why);
        return ApiResponse::not_found(i18n.translate("user.profile.not-found").as_str());
    }

    ApiResponse::ok(data.unwrap(), &i18n.translate(""))
}

pub async fn get_list_mutuals(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<FollowingDTO>> {
    let i18n = i18n!("user", lang);

    if let None = auth_context.get_user_id() {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    let create_user_id = create_object_id_option(&user_id);
    if let None = create_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }
    let mut data = DB::get(COLLECTION_MUTUALS);

    if let Some(q) = query.q {
        data = data.text(q);
    }

    let data = data
        .lookup(&[one(COLLECTION_USERS, "user_id", "_id", "follower")])
        .filter(vec![is("user_id", create_user_id.unwrap())])
        .sort(vec![("mutuals.display_name", 1)])
        .get_per_page::<FollowingDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    if let Err(why) = data {
        info!(target:"user::profile-picture::failed","{}",why);
        return ApiResponse::failed(&i18n.translate("user.profile.not-found"));
    }

    ApiResponse::ok(data.unwrap(), &i18n.translate("user.profile.not-found"))
}
