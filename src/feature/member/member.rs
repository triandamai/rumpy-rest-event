use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::{MultiFileExtractor, SingleFileExtractor};
use crate::common::orm::orm::Orm;
use crate::common::permission::permission::app;
use crate::common::utils::{
    create_object_id_option, generate_member_code, QUERY_ASC, QUERY_DESC, QUERY_LATEST,
    QUERY_OLDEST,
};
use crate::dto::coach_dto::CoachDTO;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::dto::member_dto::MemberDTO;
use crate::dto::member_log_dto::MemberLogDTO;
use crate::dto::membership_dto::MembershipDTO;
use crate::entity::detail_transaction::DetailTransaction;
use crate::entity::file_attachment::FileAttachment;
use crate::entity::member::Member;
use crate::entity::member_log::MemberLog;
use crate::entity::member_subscription::MemberSubscription;
use crate::entity::transaction::Transaction;
use crate::feature::member::member_model::{CreateMemberRequest, UpdateMemberRequest};
use crate::translate;
use axum::extract::{ Path, Query, State};
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use log::info;
use std::str::FromStr;
use validator::Validate;

pub async fn get_list_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<MemberDTO>> {
    info!(target: "member::list", "{} trying get list member",auth_context.claims.sub);
    if !auth_context.authorize(app::member::READ) {
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
        .join_one("account", "created_by_id", "_id", "created_by")
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
    if !auth_context.authorize(app::member::READ) {
        info!(target: "member::detail", "{} not permitted",auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    if auth_context.branch_id.is_none() {
        info!(target: "member::detail", "not permitted because branch not found");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(member_id.as_str());
    if id.is_none() {
        info!(target: "member::detail", "failed CREATE ObjectId");
        return ApiResponse::un_authorized(translate!("member.not-found", lang).as_str());
    }

    let find_product = Orm::get("member")
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .join_one("member-subscription", "_id", "member_id", "subscription")
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

pub async fn get_member_by_nfc(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(nfc_id): Path<String>,
) -> ApiResponse<MemberDTO> {
    if auth_context.authorize(app::member::READ) {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let find_member = Orm::get("member")
        .filter_string("nfc_id", Some("$eq"), nfc_id.as_str())
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .join_one("member-subscription", "_id", "member_id", "subscription")
        .one::<MemberDTO>(&state.db)
        .await;
    if find_member.is_err() {
        return ApiResponse::not_found(translate!("member.not-found", lang).as_str());
    }
    let find_member = find_member.unwrap();

    ApiResponse::ok(find_member, translate!("member.found", lang).as_str())
}

pub async fn create_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateMemberRequest>,
) -> ApiResponse<MemberDTO> {
    info!(target: "member::CREATE", "{} trying CREATE member",auth_context.claims.sub);
    if !auth_context.authorize(app::member::CREATE) {
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

    let membership_id = match body.coach_id.clone() {
        None => None,
        Some(coach_id) => create_object_id_option(coach_id.as_str()),
    };
    if membership_id.is_none() {
        info!(target: "member::CREATE","membership doesn't exist");
        return ApiResponse::failed(translate!("coach.not-found", lang).as_str());
    }
    let find_membership = Orm::get("membership")
        .filter_object_id("_id", &membership_id.unwrap())
        .one::<MembershipDTO>(&state.db)
        .await;
    if find_membership.is_err() {
        info!(target: "member::CREATE","membership doesn't exist");
        return ApiResponse::failed(translate!("coach.not-found", lang).as_str());
    }

    let coach_id = match body.coach_id.clone() {
        None => None,
        Some(coach_id) => create_object_id_option(coach_id.as_str()),
    };

    if coach_id.is_none() {
        info!(target: "member::CREATE","coach doesn't exist");
        return ApiResponse::failed(translate!("coach.not-found", lang).as_str());
    }
    let find_coach = Orm::get("coach")
        .filter_object_id("_id", &coach_id.unwrap())
        .one::<CoachDTO>(&state.db)
        .await;
    if find_coach.is_err() {
        info!(target: "member::CREATE","coach doesn't exist");
        return ApiResponse::failed(translate!("coach.not-found", lang).as_str());
    }

    let membership = find_membership.unwrap();
    let coach = find_coach.unwrap();
    let current_time = DateTime::now();
    let member_code = generate_member_code(body.full_name.as_str());
    let member = Member {
        id: Some(ObjectId::new()),
        member_code,
        branch_id: auth_context.branch_id,
        membership_id,
        created_by_id: auth_context.user_id,
        coach_id,
        full_name: body.full_name.clone(),
        gender: body.gender.clone(),
        email: body.email.clone(),
        identity_number: body.identity_number.clone(),
        nfc_number: Some(body.nfc_id.clone()),
        phone_number: body.phone_number.clone(),
        is_member: true,
        created_at: current_time,
        updated_at: current_time,
        deleted: false,
    };

    let subscription = MemberSubscription {
        id: Some(ObjectId::new()),
        member_id: Some(member.id.clone().unwrap()),
        membership_id,
        amount: membership.price,
        quota: membership.quota.clone(),
        create_at: current_time,
        update_at: current_time,
    };

    let transaction = Transaction {
        id: Some(ObjectId::new()),
        branch_id: Some(auth_context.branch_id.unwrap()),
        member_id: Some(member.id.clone().unwrap()),
        total_price_before_discount: membership.price,
        notes: "Pembelian paket langganan".to_string(),
        total_price: membership.price,
        total_discount: 0.0,
        is_membership: false,
        created_by_id: None,
        created_at: current_time,
        updated_at: current_time,
        deleted: false,
    };

    let detail_transaction = DetailTransaction {
        id: Some(ObjectId::new()),
        product_id: membership_id,
        kind: "MEMBERSHIP".to_string(),
        notes: format!("Paket langganan {}", membership.name),
        quantity: 1,
        total: membership.price,
        created_at: current_time,
        updated_at: current_time,
        deleted: false,
    };

    let session = state.db.start_session().await;
    if session.is_err() {
        info!(target:"stock::UPDATE","failed to CREATE trx session");
        return ApiResponse::failed(translate!("stock.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _start = session.start_transaction().await;

    let save_member = Orm::insert("member")
        .one_with_session(&member, &state.db, &mut session)
        .await;

    if save_member.is_err() {
        let _abort = session.abort_transaction().await;
        info!(target: "member::CREATE", "{}",save_member.unwrap_err());
        return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
    }

    let save_subscription = Orm::insert("member-subscription")
        .one_with_session(&subscription, &state.db, &mut session)
        .await;

    if save_subscription.is_err() {
        let _abort = session.abort_transaction().await;
        info!(target: "member::CREATE", "{}",save_subscription.unwrap_err());
        return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
    }

    let save_transaction = Orm::insert("transaction")
        .one_with_session(&transaction, &state.db, &mut session)
        .await;

    if save_transaction.is_err() {
        let _abort = session.abort_transaction().await;
        info!(target: "member::CREATE", "{}",save_transaction.unwrap_err());
        return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
    }

    let save_detail_transaction = Orm::insert("detail-transaction")
        .one_with_session(&detail_transaction, &state.db, &mut session)
        .await;

    if save_detail_transaction.is_err() {
        let _abort = session.abort_transaction().await;
        info!(target: "member::CREATE", "{}",save_detail_transaction.unwrap_err());
        return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
    }

    let _commit = session.commit_transaction().await;

    let mut dto = member.to_dto();
    let mut subs = subscription.to_dto();
    subs.membership = Some(membership);
    dto.subscription = Some(subs);
    dto.coach = Some(coach);
    ApiResponse::ok(dto, translate!("member.create.success", lang).as_str())
}

pub async fn update_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
    Json(body): Json<UpdateMemberRequest>,
) -> ApiResponse<MemberDTO> {
    info!(target: "member::UPDATE", "{} trying get list member",auth_context.claims.sub);

    if !auth_context.authorize(app::member::UPDATE) {
        info!(target: "member::UPDATE", "{} not permitted",auth_context.claims.sub);
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
        info!(target: "member::UPDATE", "failed CREATE ObjectId");
        return ApiResponse::un_authorized(translate!("member.not-found", lang).as_str());
    }

    let session = state.db.start_session().await;
    if session.is_err() {
        return ApiResponse::failed(translate!("member.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _start = session.start_transaction().await;

    let find_member = Orm::get("member")
        .filter_object_id("_id", &member_id.unwrap())
        .join_one("account", "create_by_id", "_id", "created_by")
        .join_one("coach", "coach_id", "_id", "coach")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .one::<MemberDTO>(&state.db)
        .await;
    if find_member.is_err() {
        info!(target: "member::UPDATE", "{}",find_member.unwrap_err());
        let _abort = session.abort_transaction().await;
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

    if body.identity_number.is_some() {
        member.identity_number = body.identity_number.clone();
        save = save.set_str(
            "identity_number",
            body.identity_number.clone().unwrap().as_str(),
        );
    }

    if body.nfc_id.is_some() {
        member.nfc_id = body.nfc_id.clone();
        save = save.set_str("nfc_id", body.nfc_id.clone().unwrap().as_str());
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

    let find_subscription = Orm::get("member-subscription")
        .filter_object_id("member_id", &member_id.unwrap())
        .one::<MemberSubscription>(&state.db)
        .await;

    let is_has_subs = find_subscription.is_ok();
    let mut subscription = find_subscription.unwrap_or_else(|_| MemberSubscription {
        id: Some(ObjectId::new()),
        member_id,
        membership_id: None,
        amount: 0.0,
        quota: 0,
        create_at: DateTime::now(),
        update_at: DateTime::now(),
    });

    let mut should_update_membership = false;
    if body.membership_id.is_some() {
        should_update_membership = match body.membership_id {
            None => false,
            Some(id) => ObjectId::from_str(id.as_str()).map_or_else(
                |_| false,
                |obj_id| obj_id == subscription.membership_id.unwrap(),
            ),
        };
    }

    let current_time = DateTime::now();
    if should_update_membership {
        let find_membership = Orm::get("membership")
            .filter_object_id("_id", &subscription.membership_id.unwrap())
            .one::<MembershipDTO>(&state.db)
            .await;

        if find_membership.is_err() {
            info!(target: "member::UPDATE", "{}",find_membership.unwrap_err());
            let _abort = session.abort_transaction().await;
            return ApiResponse::failed(translate!("member.update.failed", lang).as_str());
        }

        let membership = find_membership.unwrap();

        let transaction = Transaction {
            id: Some(ObjectId::new()),
            branch_id: Some(auth_context.branch_id.unwrap()),
            member_id: Some(member.id.clone().unwrap()),
            total_price_before_discount: membership.price,
            notes: "Pembelian pergantian paket langganan".to_string(),
            total_price: membership.price,
            total_discount: 0.0,
            is_membership: false,
            created_by_id: None,
            created_at: current_time,
            updated_at: current_time,
            deleted: false,
        };

        let detail_transaction = DetailTransaction {
            id: Some(ObjectId::new()),
            product_id: membership.id,
            kind: "MEMBERSHIP".to_string(),
            notes: format!("Paket langganan {}", membership.name),
            quantity: 1,
            total: membership.price,
            created_at: current_time,
            updated_at: current_time,
            deleted: false,
        };

        let save_transaction = Orm::insert("transaction")
            .one_with_session(&transaction, &state.db, &mut session)
            .await;

        if save_transaction.is_err() {
            let _abort = session.abort_transaction().await;
            info!(target: "member::create", "{}",save_transaction.unwrap_err());
            return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
        }

        let save_detail_transaction = Orm::insert("detail-transaction")
            .one_with_session(&detail_transaction, &state.db, &mut session)
            .await;

        if save_detail_transaction.is_err() {
            let _abort = session.abort_transaction().await;
            info!(target: "member::create", "{}",save_detail_transaction.unwrap_err());
            return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
        }

        //when already has subscription only UPDATE amount
        //when no subs exist insert new one
        //rollback when failed
        if is_has_subs {
            let save_subscription = Orm::update("member-subscription")
                .inc(doc! {
                    "quota": membership.quota,
                })
                .filter_object_id("_id", &membership.id.unwrap())
                .execute_one_with_session(&state.db, &mut session)
                .await;

            if save_subscription.is_err() {
                let _abort = session.abort_transaction().await;
                info!(target: "member::CREATE", "{}",save_subscription.unwrap_err());
                return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
            }
        } else {
            //make sure UPDATE the struct
            subscription.amount = membership.price;
            subscription.quota = membership.quota;
            subscription.membership_id = membership.id;

            let save_subscription = Orm::insert("member-subscription")
                .one_with_session(subscription, &state.db, &mut session)
                .await;
            if save_subscription.is_err() {
                let _abort = session.abort_transaction().await;
                info!(target: "member::CREATE", "{}",save_subscription.unwrap_err());
                return ApiResponse::failed(translate!("member.create.failed", lang).as_str());
            }
        }
    }

    //finally notify member has been updated
    let save_data = save
        .filter_object_id("_id", &member_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .execute_many_with_session(&state.db, &mut session)
        .await;

    if save_data.is_err() {
        info!(target: "member::update", "{}", save_data.unwrap_err());

        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(translate!("member.update.failed", lang).as_str());
    }
    let _commit = session.commit_transaction().await;
    info!(target: "member::update", "Successfully UPDATE member");
    ApiResponse::ok(member, translate!("member.update.success", lang).as_str())
}

pub async fn delete_member(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "member::delete", "{} trying get list member",auth_context.claims.sub);
    if !auth_context.authorize(app::member::DELETE) {
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
    multipart: SingleFileExtractor,
) -> ApiResponse<FileAttachmentDTO> {
    if !auth_context.authorize(app::member::UPDATE) {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = multipart.validate_body();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("member.profile-picture.failed", lang).as_str(),
        );
    }

    let extract = multipart;

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

    let extract = extract.file();
    let minio = MinIO::new().await;
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
        let _delete_existing = minio
            .delete_file(attachment.filename.clone(), bucket_name.clone())
            .await;
    }

    //upload new
    let minio = minio
        .upload_file(
            extract.temp_path.clone(),
            bucket_name,
            extract.filename.clone(),
        )
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
                .set_str("filename", &extract.filename.as_str())
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

//activity
pub async fn get_member_activity(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(member_id): Path<String>,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<MemberLogDTO>> {
    if !auth_context.authorize(app::member::READ) {
        return ApiResponse::bad_request(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        return ApiResponse::bad_request(translate!("unauthorized", lang).as_str());
    }
    let member_id = create_object_id_option(member_id.as_str());
    if member_id.is_none() {
        return ApiResponse::not_found(translate!("member.profile-picture.failed", lang).as_str());
    }
    let find = Orm::get("member-log")
        .filter_object_id("member_id", &member_id.unwrap())
        .join_one("member", "member_id", "_id", "member")
        .join_one("account", "created_by_id", "_id", "created_by")
        .join_many("file-attachment", "_id", "ref_id", "attachments")
        .group_by_desc("created_at")
        .pageable::<MemberLogDTO>(query.page.unwrap_or(0), query.size.unwrap_or(0), &state.db)
        .await;

    if find.is_err() {
        return ApiResponse::bad_request(translate!("bad-request", lang).as_str());
    }

    ApiResponse::ok(find.unwrap(), translate!("", lang).as_str())
}

pub async fn upload_progress(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    multipart: MultiFileExtractor,
) -> ApiResponse<MemberLogDTO> {
    if !auth_context.authorize(app::member::UPDATE) {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = multipart.validate();
    if validate.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }

    let session = &state.db.start_session().await;
    if session.is_err() {
        return ApiResponse::failed(translate!("start-session-failed", lang).as_str());
    }
    let session = state.db.start_session().await;
    if session.is_err() {
        return ApiResponse::failed(translate!("member.update.failed", lang).as_str());
    }
    let mut session = session.unwrap();
    let _start = session.start_transaction().await;

    let member_id = create_object_id_option(multipart.ref_id.as_str());

    if member_id.is_none() {
        return ApiResponse::un_authorized(
            translate!("member.profile-picture.failed", lang).as_str(),
        );
    }
    let minio = MinIO::new().await;

    let member_log = MemberLog {
        id: Some(ObjectId::new()),
        member_id: member_id,
        created_by_id: auth_context.user_id,
        branch_id: auth_context.branch_id,
        name: "Update progres harian".to_string(),
        value: "".to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let image_data_temp = multipart
        .temp_file
        .iter()
        .map(|(key, file)| FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: member_id,
            filename: file.filename.clone(),
            mime_type: file.mime_type.clone(),
            extension: file.extension.clone(),
            kind: if key == "data-image" {
                "MEMBER-PROGRESS".to_string()
            } else {
                "MEMBER-BODY-IMAGE".to_string()
            },
            create_at: DateTime::now(),
            updated_at: DateTime::now(),
        })
        .collect::<Vec<FileAttachment>>();

    let data = image_data_temp.clone();
    let save_data_image = Orm::insert("file-attachment")
        .many_with_session::<FileAttachment>(data, &state.db, &mut session)
        .await;

    if save_data_image.is_err() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(translate!("", lang).as_str());
    }

    //upload data image
    let upload = multipart.temp_file.get("data-image");
    if upload.is_none() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::bad_request(
            translate!("upload-profile-picture.failed", lang).as_str(),
        );
    }
    let upload = upload.unwrap();
    let upload_data = minio
        .upload_file(
            upload.temp_path.clone(),
            "member-log".to_string(),
            upload.filename.clone(),
        )
        .await;

    if upload_data.is_err() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::bad_request(
            translate!("upload-profile-picture.failed", lang).as_str(),
        );
    }

    //upload body image
    let upload = multipart.temp_file.get("body-image");
    if upload.is_none() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::bad_request(
            translate!("upload-profile-picture.failed", lang).as_str(),
        );
    }
    let upload = upload.unwrap();
    let upload_data = minio
        .upload_file(
            upload.temp_path.clone(),
            "member-log".to_string(),
            upload.filename.clone(),
        )
        .await;

    if upload_data.is_err() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::bad_request(
            translate!("upload-profile-picture.failed", lang).as_str(),
        );
    }

    let save = Orm::insert("member-log")
        .one_with_session(&member_log, &state.db, &mut session)
        .await;

    if save.is_err() {
        let _abort = session.abort_transaction().await;
        return ApiResponse::failed(translate!("member.profile-picture.failed", lang).as_str());
    }

    let _commit = session.commit_transaction().await;

    let mut dto = member_log.to_dto();
    dto.attachments = Some(image_data_temp.iter().map(|v|v.clone().to_dto()).collect());

    ApiResponse::ok(
        dto,
        translate!("member.profile-picture.failed", lang).as_str(),
    )
}
