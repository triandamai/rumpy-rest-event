use axum::extract::{Path, Query, State};
use bcrypt::DEFAULT_COST;
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use log::info;
use validator::Validate;

use crate::common::api_response::{PaginationRequest, PagingResponse};
use crate::common::constant::{BUCKET_PROFILE_PICTURE, COLLECTION_FOLLOWER, COLLECTION_USER, COLLECTION_USER_PROFILE, PROVIDER_BASIC, REDIS_KEY_USER_ID};
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::mongo::filter::{equal, is};
use crate::common::mongo::lookup::one;
use crate::common::mongo::DB;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::utils::create_object_id_option;
use crate::dto::following_dto::FollowingDTO;
use crate::dto::user_dto::UserDTO;
use crate::entity::following::Following;
use crate::entity::user_metadata::UserMetaData;
use crate::feature::user::user_model::{ChangePasswordRequest, UserWithProfiledResponse};
use crate::{
    common::{
        api_response::ApiResponse, app_state::AppState, constant::REDIS_KEY_USER_EMAIL,
        jwt::AuthContext, lang::Lang,
    },
    entity::{profile_picture::ProfilePicture, user::User},
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

    let find_user = DB::get(COLLECTION_USER)
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

pub async fn change_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<ChangePasswordRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"user::change-password::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(
            err,
            i18n.translate("user.change-password.validation.error")
                .as_str(),
        );
    }

    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::change-password::failed","session not found {:?}",auth_context.session);
        return ApiResponse::failed(
            i18n.translate("user.change-password.user.not-found")
                .as_str(),
        );
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::change-password::failed","user not found {:?}",err);
        return ApiResponse::failed(
            i18n.translate("user.change-password.user.not-found")
                .as_str(),
        );
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
            return ApiResponse::failed(
                i18n.translate("user.change-password.user.invalid").as_str(),
            );
        }
        if !verify.unwrap() {
            info!(target:"user::change-password::failed","current password invalid");
            return ApiResponse::failed(
                i18n.translate("user.change-password.user.invalid").as_str(),
            );
        }
    } else {
        info!(target:"user::change-password","user provider has not BASIC, skip verify current password");
        providers.push(PROVIDER_BASIC.to_string());
        meta_data.providers = Some(providers);
        user.user_meta_data = Some(meta_data);
    }

    let create_password = bcrypt::hash(body.new_password, DEFAULT_COST);
    if let Err(err) = create_password {
        info!(target:"user::change-password::failed","error while create has password {:?}",err);
        return ApiResponse::failed(i18n.translate("user.change-password.user.invalid").as_str());
    }

    let update_password = DB::update(COLLECTION_USER)
        .set(doc! {
            "password":create_password.unwrap(),
            "updated_at":DateTime::now()
        })
        .filter(vec![equal("_id", &user.id)])
        .execute(&state.db)
        .await;

    if let Err(err) = update_password {
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

    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::access_denied(
            i18n.translate("user.update-profile-picture.not-found")
                .as_str(),
        );
    }
    let user_email = user_email.unwrap();
    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
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
    let upload = MinIO::new()
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

    let profile_picture = ProfilePicture {
        id: Some(ObjectId::new()),
        mime_type: upload_file.mime_type.clone(),
        file_name: part_file_name.clone(),
        bucket_name: BUCKET_PROFILE_PICTURE.to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    let insert_profile_picture = DB::update(COLLECTION_USER)
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
) -> ApiResponse<User> {
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

    let data = DB::get(COLLECTION_USER)
        .lookup(&[one("user-profile", "_id", "_id", "profile")])
        .filter(vec![is("_id", create_user_id.unwrap())])
        .get_one::<UserDTO>(&state.db)
        .await;
    if let Err(why) = data {
        info!(target:"user::profile-picture::failed","{}",why);
        return ApiResponse::not_found(i18n.translate("user.profile.not-found").as_str());
    }
    ApiResponse::failed("")
}

pub async fn get_list_follower(
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
    let mut data = DB::get(COLLECTION_FOLLOWER);

    if let Some(q) = query.q {
        data = data.text(q);
    }

    let data = data
        .lookup(&[one("user", "user_id", "_id", "follower")])
        .filter(vec![is("user_id", create_user_id.unwrap())])
        .sort(vec![("follower.display_name", 1)])
        .get_per_page::<FollowingDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    if let Err(why) = data {
        info!(target:"user::profile-picture::failed","{}",why);
        return ApiResponse::failed(&i18n.translate("user.profile.not-found"));
    }

    ApiResponse::ok(data.unwrap(), &i18n.translate("user.profile.not-found"))
}

pub async fn get_list_following(
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
    let mut data = DB::get(COLLECTION_FOLLOWER);

    if let Some(q) = query.q {
        data = data.text(q);
    }

    let data = data
        .lookup(&[one("user", "follower_id", "_id", "user")])
        .filter(vec![is("user_id", create_user_id.unwrap())])
        .sort(vec![("user.display_name", 1)])
        .get_per_page::<FollowingDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    if let Err(why) = data {
        info!(target:"user::profile-picture::failed","{}",why);
        return ApiResponse::failed(&i18n.translate("user.profile.not-found"));
    }

    ApiResponse::ok(data.unwrap(), &i18n.translate("user.profile.not-found"))
}

pub async fn follow_user(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);
    let create_user_id = create_object_id_option(&user_id);
    let current_user_id = auth_context.get_user_id();

    if let None = create_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    if let None = current_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![
            is("user_id", create_user_id.unwrap()),
            is("follower_id", current_user_id.unwrap()),
        ])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Some(_user) = find_user {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::ok(
            "OK".to_string(),
            i18n.translate("user.follow.exist").as_str(),
        );
    }

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let following = Following {
        user_id: create_user_id,
        follower_id: current_user_id,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let save_following = DB::insert(COLLECTION_FOLLOWER)
        .one_with_session(following, &state.db, &mut session)
        .await;
    if let Err(err) = save_following {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let increment_follower = DB::update(COLLECTION_USER_PROFILE)
        .inc(doc! {
            "follower":1
        })
        .filter(vec![is("_id", create_user_id.unwrap())])
        .execute_with_session(&state.db, &mut session)
        .await;
    if let Err(err) = increment_follower {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let increment_following = DB::update(COLLECTION_USER_PROFILE)
        .inc(doc! {
            "following":1
        })
        .filter(vec![is("_id", current_user_id.unwrap())])
        .execute_with_session(&state.db, &mut session)
        .await;
    if let Err(err) = increment_following {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;
    ApiResponse::ok("OK".to_string(), &i18n.translate("user.profile.not-found"))
}

pub async fn unfollow_user(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);
    let create_user_id = create_object_id_option(&user_id);
    let current_user_id = auth_context.get_user_id();

    if let None = create_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    if let None = current_user_id {
        info!(target:"user::profile-picture::failed","session not found");
        return ApiResponse::failed(i18n.translate("user.profile.not-found").as_str());
    }

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![
            is("user_id", create_user_id.unwrap()),
            is("follower_id", current_user_id.unwrap()),
        ])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let None = find_user {
        info!(target:"user::profile-picture::failed","already unfollow");
        return ApiResponse::ok(
            "OK".to_string(),
            i18n.translate("user.follow.exist").as_str(),
        );
    }

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_following = DB::delete(COLLECTION_FOLLOWER)
        .filter(vec![
            is("user_id", create_user_id.unwrap()),
            is("follower_id", current_user_id.unwrap()),
        ])
        .one_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = save_following {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let deleted_count = save_following.unwrap();
    if deleted_count < 1 {
        info!(target:"stock::update","deleted count {:?}", deleted_count);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let increment_follower = DB::update(COLLECTION_USER_PROFILE)
        .inc(doc! {
            "follower":-1
        })
        .filter(vec![is("_id", create_user_id.unwrap())])
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = increment_follower {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let increment_following = DB::update(COLLECTION_USER_PROFILE)
        .inc(doc! {
            "following":-1
        })
        .filter(vec![is("_id", current_user_id.unwrap())])
        .execute_with_session(&state.db, &mut session)
        .await;
    if let Err(err) = increment_following {
        info!(target:"stock::update","{:?}",err);
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;
    ApiResponse::ok("OK".to_string(), &i18n.translate("user.profile.not-found"))
}
