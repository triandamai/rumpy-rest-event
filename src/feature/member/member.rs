use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::multipart_file::MultipartFile;
use crate::common::orm::orm::Orm;
use crate::common::utils::{
    create_object_id_option, generate_member_code, QUERY_ASC, QUERY_DESC, QUERY_LATEST,
    QUERY_OLDEST,
};
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::dto::member_dto::MemberDTO;
use crate::entity::file_attachment::FileAttachment;
use crate::entity::member::Member;
use crate::feature::member::member_model::{CreateMemberRequest, UpdateMemberRequest};
use crate::translate;
use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;
use chrono::NaiveDate;
use log::info;
use validator::Validate;

pub async fn get_list_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<MemberDTO>> {
    info!(target: "member::list", "{} trying get list member",auth_context.claims.sub);
    if !auth_context.authorize("app::member::read") {
        info!(target: "member::list", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "member::list","not permitted branch");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default.clone());
    let mut get = Orm::get("member");

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
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .pageable::<MemberDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;

    info!(target: "member::list", "successfully get list member");
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("member.list.success", lang).as_str(),
    )
}

pub async fn get_detail_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
) -> ApiResponse<MemberDTO> {
    info!(target: "member::detail", "{} trying get detail member",auth_context.claims.sub);
    if !auth_context.authorize("app::member::read") {
        info!(target: "member::detail", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "member::detail", "not permitted because branch not found");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(member_id.as_str());
    if id.is_none() {
        info!(target: "member::detail", "failed create ObjectId");
        return ApiResponse::un_authorized(translate!("member.not-found", lang).as_str());
    }

    let find_product = Orm::get("member")
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .one::<MemberDTO>(&state.db)
        .await;

    if find_product.is_err() {
        info!(target: "member::detail","Data member not found");
        return ApiResponse::not_found(translate!("member.not-found", lang).as_str());
    }

    ApiResponse::ok(
        find_product.unwrap(),
        translate!("member.found", lang).as_str(),
    )
}

pub async fn create_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    body: Json<CreateMemberRequest>,
) -> ApiResponse<MemberDTO> {
    info!(target: "member::create", "{} trying create member",auth_context.claims.sub);
    if !auth_context.authorize("app::member::write") {
        info!(target: "member::list", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let dob = body.date_of_birth.clone().map_or_else(
        || None,
        |v| NaiveDate::parse_from_str(v.as_str(), "%Y-%m-%d").map_or(None, |v| Some(v)),
    );
    let coach_id = match body.coach_id.clone() {
        None => None,
        Some(coach_id) => create_object_id_option(coach_id.as_str()),
    };

    let member_code = generate_member_code(body.full_name.as_str());
    let product = Member {
        id: Some(ObjectId::new()),
        member_code: member_code,
        branch_id: auth_context.branch_id,
        created_by_id: auth_context.user_id,
        coach_id: coach_id,
        full_name: body.full_name.clone(),
        gender: body.gender.clone(),
        email: body.email.clone(),
        date_of_birth: dob,
        phone_number: body.phone_number.clone(),
        is_member: true,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let save = Orm::insert("member").one(&product, &state.db).await;
    if save.is_err() {
        info!(target: "member::create", "{}",save.unwrap_err());
        return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
    }
    ApiResponse::ok(
        product.to_dto(),
        translate!("member.create.success", lang).as_str(),
    )
}

pub async fn update_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
    body: Json<UpdateMemberRequest>,
) -> ApiResponse<MemberDTO> {
    info!(target: "member::update", "{} trying get list member",auth_context.claims.sub);
    if !auth_context.authorize("app::member::write") {
        info!(target: "member::update", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("member.update.failed", lang).as_str(),
        );
    }

    let member_id = create_object_id_option(member_id.as_str());
    if member_id.is_none() {
        info!(target: "member::update", "failed create ObjectId");
        return ApiResponse::un_authorized(translate!("member.not-found", lang).as_str());
    }

    let find_member = Orm::get("member")
        .filter_object_id("_id", &member_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .one::<MemberDTO>(&state.db)
        .await;
    if find_member.is_err() {
        info!(target: "member::update", "{}",find_member.unwrap_err());
        return ApiResponse::not_found(translate!("member.not-found", lang).as_str());
    }
    let mut member = find_member.unwrap();

    let mut save = Orm::update("member");
    if body.full_name.is_some() {
        member.full_name = body.full_name.clone().unwrap();
        save = save.set_str("full_name", &body.full_name.clone().unwrap());
    }

    if body.email.is_some() {
        member.email = body.email.clone();
        save = save.set_str("email", &body.email.clone().unwrap());
    }

    if body.gender.is_some() {
        member.gender = body.gender.clone();
        save = save.set_str("gender", &body.gender.clone().unwrap());
    }

    if body.date_of_birth.is_some() {
        let dob = body.date_of_birth.clone().map_or_else(
            || None,
            |v| NaiveDate::parse_from_str(v.as_str(), "%Y-%m-%d").map_or(None, |v| Some(v)),
        );
        member.date_of_birth = dob;
        save = save.set_naive_date("date_of_birth", &dob.unwrap());
    }

    if body.coach_id.is_some() {
        let id = match body.coach_id.clone() {
            None => None,
            Some(v) => create_object_id_option(v.as_str()),
        };
        member.coach_id = id;
        if id.is_some() {
            save = save.set_object_id("coach_id", &id.unwrap());
        }
    }

    if body.phone_number.is_some() {
        member.phone_number = body.phone_number.clone();
        if body.phone_number.is_some() {
            save = save.set_str("phone_number", &body.phone_number.clone().unwrap());
        }
    }

    let save_data = save
        .filter_object_id("_id", &member_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .execute_one(&state.db)
        .await;

    if save_data.is_err() {
        info!(target: "member::update", "{}",save_data.unwrap_err());
        return ApiResponse::failed(translate!("member.update.failed", lang).as_str());
    }
    info!(target: "member::update", "Successfully update member");
    ApiResponse::ok(member, translate!("member.update.success", lang).as_str())
}

pub async fn delete_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "member::delete", "{} trying get list member",auth_context.claims.sub);
    if !auth_context.authorize("app::member::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(member_id.as_str());
    if id.is_none() {
        return ApiResponse::un_authorized(translate!("member.not-found", lang).as_str());
    }

    let update = Orm::update("member")
        .filter_object_id("_id", &id.unwrap())
        .set_bool("deleted", true)
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        return ApiResponse::failed(translate!("member.delete.failed", lang).as_str());
    }

    ApiResponse::ok(
        "OK".to_string(),
        translate!("member.delete.success", lang).as_str(),
    )
}

pub async fn update_profile_picture(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    multipart: Multipart,
) -> ApiResponse<FileAttachmentDTO> {
    if !auth_context.authorize("app::member::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let extract = MultipartFile::extract_multipart(multipart).await;

    let validate = extract.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("member.profile-picture.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(extract.ref_id.as_str());
    if user_id.is_none() {
        return ApiResponse::not_found(
            translate!("member.profile-picture.not-found", lang).as_str(),
        );
    }
    let find_exist_profile_picture = Orm::get("file-attachment")
        .filter_object_id("ref_id", &user_id.unwrap())
        .one::<FileAttachment>(&state.db)
        .await;

    let minio = MinIO::new().await;
    let mut filename = format!("{}.{}", extract.filename, extract.extension);
    let is_file_exists = find_exist_profile_picture.is_ok();
    let bucket_name = "member-profile-picture".to_string();

    let attachment = match find_exist_profile_picture {
        Ok(v) => v,
        Err(_) => FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: create_object_id_option(extract.ref_id.as_str()),
            filename: extract.filename.clone(),
            mime_type: extract.mime_type.clone(),
            extension: extract.extension.clone(),
            kind: "MEMBER".to_string(),
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
        .upload_file(extract.temp_path.clone(), bucket_name, filename.clone())
        .await;

    if minio.is_err() {
        let err = minio.unwrap_err();
        info!(target: "upload-profile-picture", "{}", err);
        let _remove = extract.remove_file();
        return ApiResponse::failed(translate!("member.profile-picture.failed", lang).as_str());
    }

    let mut error_message = String::new();
    let success = match is_file_exists {
        true => {
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
        let _remove = extract.remove_file();
        return ApiResponse::failed(translate!("member.profile-picture.failed", lang).as_str());
    }

    let _remove = extract.remove_file();
    ApiResponse::ok(
        attachment.to_dto(),
        translate!("member.profile-picture.success", lang).as_str(),
    )
}
