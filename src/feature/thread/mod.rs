use std::str::FromStr;

use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::constant::{
    BUCKET_THREAD, COLLECTION_RESERVE_ATTACHMENT, COLLECTION_THREAD, COLLECTION_THREAD_VOTE,
    COLLECTION_USER, KIND_DISCUSSION, KIND_DOWN_VOTE_THREAD, KIND_PUBLIC, KIND_THREAD_ATTACHMENT,
    KIND_UP_VOTE_THREAD, REDIS_KEY_USER_EMAIL,
};
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::utils::create_object_id_option;
use crate::dto::thread_attachment_dto::ThreadAttachmentDTO;
use crate::dto::thread_dto::ThreadDTO;

use crate::common::mongo::filter::{equal, is, is_in, FilterGroup};
use crate::common::mongo::lookup::{one, one_merge_to};
use crate::common::mongo::DB;
use crate::entity::thread::Thread;
use crate::entity::thread_attachment::ThreadAttachment;
use crate::entity::thread_vote::ThreadVote;
use crate::entity::user::User;
use crate::i18n;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use log::info;
use thread_model::{CreatedThreadRequest, UpdateThreadRequest};
use validator::Validate;

pub mod thread_model;

pub async fn get_list_public_thread(
    state: State<AppState>,
    lang: Lang,
    _auth_context: AuthContext,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let mut find_thread = DB::get(COLLECTION_THREAD);
    if let Some(text) = query.q.clone() {
        find_thread = find_thread.text(text.as_str());
    }

    if let Some((column, order)) = query.get_sorted() {
        find_thread = if order == "ASC" {
            find_thread.sort(vec![(&column.clone(), 1)])
        } else {
            find_thread.sort(vec![(&column.clone(), -1)])
        };
    } else {
        find_thread = find_thread.sort(vec![("created_at", -1)]);
    }

    let find_thread = find_thread
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
            one("thread", "top_thread_id", "_id", "top_thread"),
            one_merge_to(
                "user",
                "top_thread.created_by_id",
                "_id",
                "created_by",
                "top_thread",
            ),
        ])
        .filter(vec![
            equal("reply_to_thread_id", None::<i32>),
            equal("kind", KIND_PUBLIC),
        ])
        .get_per_page::<ThreadDTO>(page, size, &state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("get.public.thread.not-found"));
    }

    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("get.public.thread.success"),
    )
}

pub async fn get_list_discussion_thread(
    state: State<AppState>,
    lang: Lang,
    _auth_context: AuthContext,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let mut find_thread = DB::get(COLLECTION_THREAD);
    if let Some(text) = query.q.clone() {
        find_thread = find_thread.text(text.as_str());
    }

    if let Some((column, order)) = query.get_sorted() {
        find_thread = if order == "ASC" {
            find_thread.sort(vec![(&column.clone(), 1)])
        } else {
            find_thread.sort(vec![(&column.clone(), -1)])
        };
    } else {
        find_thread = find_thread.sort(vec![("created_at", -1)]);
    }

    let find_thread = find_thread
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
            one("thread", "top_thread_id", "_id", "top_thread"),
            one_merge_to(
                "user",
                "top_thread.created_by_id",
                "_id",
                "created_by",
                "top_thread",
            ),
            one("thread", "top_thread_id", "_id", "top_thread"),
            one_merge_to(
                "user",
                "top_thread.created_by_id",
                "_id",
                "created_by",
                "top_thread",
            ),
        ])
        .filter(vec![
            equal("reply_to_thread_id", None::<i32>),
            equal("kind", KIND_DISCUSSION),
        ])
        .get_per_page::<ThreadDTO>(page, size, &state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("get.public.thread.not-found"));
    }

    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("get.public.thread.success"),
    )
}

pub async fn get_list_user_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(user_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool
    if let None = auth_context.get_user_id() {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let create_user_id = create_object_id_option(&user_id);
    if let None = create_user_id {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let mut find_thread = DB::get(COLLECTION_THREAD);
    if let Some(text) = query.q.clone() {
        find_thread = find_thread.text(text.as_str());
    }

    if let Some((column, order)) = query.get_sorted() {
        find_thread = if order == "ASC" {
            find_thread.sort(vec![(&column.clone(), 1)])
        } else {
            find_thread.sort(vec![(&column.clone(), -1)])
        };
    } else {
        find_thread = find_thread.sort(vec![("created_at", -1)]);
    }

    let find_thread = find_thread
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
            one("thread", "top_thread_id", "_id", "top_thread"),
            one_merge_to(
                "user",
                "top_thread.created_by_id",
                "_id",
                "created_by",
                "top_thread",
            ),
        ])
        .filter(vec![is("created_by_id", create_user_id)])
        .get_per_page::<ThreadDTO>(page, size, &state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("get.public.thread.not-found"));
    }

    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("get.public.thread.success"),
    )
}

pub async fn get_list_comment_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool
    if let None = auth_context.get_user_id() {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let create_thread_id = create_object_id_option(&thread_id);
    if let None = create_thread_id {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let mut find_thread = DB::get(COLLECTION_THREAD);
    if let Some(text) = query.q.clone() {
        find_thread = find_thread.text(text.as_str());
    }

    if let Some((column, order)) = query.get_sorted() {
        find_thread = if order == "ASC" {
            find_thread.sort(vec![(&column.clone(), 1)])
        } else {
            find_thread.sort(vec![(&column.clone(), -1)])
        };
    } else {
        find_thread = find_thread.sort(vec![("created_at", -1)]);
    }

    let find_thread = find_thread
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
        ])
        .filter(vec![is("reply_to_thread_id", create_thread_id)])
        .get_per_page::<ThreadDTO>(page, size, &state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("get.public.thread.not-found"));
    }

    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("get.public.thread.success"),
    )
}

pub async fn get_detail_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<ThreadDTO> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool
    if let None = auth_context.get_user_id() {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let create_thread_id = create_object_id_option(&thread_id);
    if let None = create_thread_id {
        info!(target:"thread::list","failed to find user_id");
        return ApiResponse::failed(&i18n.translate(""));
    }

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let mut find_thread = DB::get(COLLECTION_THREAD);
    if let Some(text) = query.q.clone() {
        find_thread = find_thread.text(text.as_str());
    }

    if let Some((column, order)) = query.get_sorted() {
        find_thread = if order == "ASC" {
            find_thread.sort(vec![(&column.clone(), 1)])
        } else {
            find_thread.sort(vec![(&column.clone(), -1)])
        };
    } else {
        find_thread = find_thread.sort(vec![("created_at", -1)]);
    }

    let find_thread = find_thread
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
            one("thread", "top_thread_id", "_id", "top_thread"),
            one_merge_to(
                "user",
                "top_thread.created_by_id",
                "_id",
                "created_by",
                "top_thread",
            ),
        ])
        .filter(vec![is("_id", create_thread_id)])
        .get_one::<ThreadDTO>(&state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("get.public.thread.not-found"));
    }

    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("get.public.thread.success"),
    )
}

pub async fn upload_attachment(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    form_data: SingleFileExtractor,
) -> ApiResponse<ThreadAttachmentDTO> {
    let i18n = i18n!("user", lang);

    let validate = form_data.validate();
    if let Err(err) = validate {
        info!(target:"user::profile::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.profile.failed").as_str());
    }

    let user_email: Option<&String> = auth_context.session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","user not found {:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let file = form_data.file();

    let attachment = ThreadAttachment {
        id: Some(ObjectId::new()),
        kind: KIND_THREAD_ATTACHMENT.to_string(),
        mime_type: file.clone().mime_type,
        bucket_name: BUCKET_THREAD.to_string(),
        file_name: file.filename.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;
    let save_attachment = DB::insert(COLLECTION_RESERVE_ATTACHMENT)
        .one_with_session(attachment.clone(), &state.db, &mut session)
        .await;
    if let Err(err) = save_attachment {
        info!(target:"attachment::upload::faile","{:?}",err);
        let _ = session.abort_transaction().await;
        let _ = file.remove_file();
        return ApiResponse::failed(&i18n.translate("attachment.failed"));
    }

    let upload = MinIO::new()
        .upload_file(
            file.temp_path.clone(),
            BUCKET_THREAD.to_string(),
            file.filename.clone(),
        )
        .await;

    if let Err(err) = upload {
        info!(target:"attachment::upload::faile","{:?}",err);
        let _ = session.abort_transaction().await;
        let _ = file.remove_file();
        return ApiResponse::failed(&i18n.translate("attachment.failed"));
    }
    let _ = file.remove_file();
    let _commit = session.commit_transaction().await;

    ApiResponse::ok(attachment.into(), "message")
}

pub async fn create_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<CreatedThreadRequest>,
) -> ApiResponse<ThreadDTO> {
    let i18n = i18n!("user", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"user::profile::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.profile.failed").as_str());
    }

    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user = find_user.unwrap();

    let find_all_attachment = DB::get(COLLECTION_RESERVE_ATTACHMENT)
        .filter(vec![is_in("_id", body.attachment.clone())])
        .get_all::<ThreadAttachment>(&state.db)
        .await
        .unwrap_or(Vec::new());

    let quote = body.quote_thread_id.map_or_else(
        || None,
        |id| ObjectId::from_str(id.as_str()).map_or_else(|_| None, |v| Some(v)),
    );

    let reply = body.reply_to_thread_id.map_or_else(
        || None,
        |id| ObjectId::from_str(id.as_str()).map_or_else(|_| None, |v| Some(v)),
    );

    let thread = Thread {
        id: Some(ObjectId::new()),
        created_by_id: user.id,
        quote_thread_id: quote,
        reply_to_thread_id: reply,
        top_thread_id: None,
        kind: KIND_PUBLIC.to_string(),
        slug: body.slug,
        title: body.title,
        content: body.content,
        attachment: find_all_attachment.clone(),
        topics: Some(body.topics.clone()),
        up_vote_count: 0,
        down_vote_count: 0,
        quote_count: 0,
        reply_count: 0,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let insert_thread = DB::insert(COLLECTION_THREAD)
        .one_with_session(thread, &state.db, &mut session)
        .await;

    if let Err(err) = insert_thread {
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    //update counter
    if let Some(quote) = quote {
        let update_counter = DB::update(COLLECTION_THREAD)
            .inc(doc! {
                "quote_count":1
            })
            .filter(vec![is("_id", &quote)])
            .execute_with_session(&state.db, &mut session)
            .await;
        if update_counter.is_err() {
            let err = update_counter.unwrap_err();
            info!(target:"stock::update","insert failed {:?}",err);
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("thread::create::failed"));
        }
    }

    if let Some(reply) = reply {
        let update_counter = DB::update(COLLECTION_THREAD)
            .inc(doc! {
                "reply_count":1
            })
            .filter(vec![is("_id", &reply)])
            .execute_with_session(&state.db, &mut session)
            .await;
        if let Err(err) = update_counter {
            info!(target:"stock::update","insert failed {:?}",err);
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("thread::create::failed"));
        }
    }
    //end update counter

    let delete_all_reserve_attachment = DB::delete(COLLECTION_RESERVE_ATTACHMENT)
        .filter(vec![is_in("_id", body.attachment)])
        .many_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = delete_all_reserve_attachment {
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    for deleted in find_all_attachment {
        let _delete_from_bucket = MinIO::new()
            .delete_file(deleted.file_name, BUCKET_THREAD.to_string())
            .await;
    }

    let commit = session.commit_transaction().await;
    if let Err(err) = commit {
        info!(target:"stock::update","insert failed {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let find_thread = DB::get(COLLECTION_THREAD)
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
        ])
        .filter(vec![is("_id", &insert_thread.unwrap())])
        .get_one::<ThreadDTO>(&state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    ApiResponse::ok(
        find_thread.unwrap(),
        &i18n.translate("thread::create::success"),
    )
}

pub async fn update_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
    Json(body): Json<UpdateThreadRequest>,
) -> ApiResponse<ThreadDTO> {
    let i18n = i18n!("user", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"user::profile::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.profile.failed").as_str());
    }

    let create_thread_id = create_object_id_option(&thread_id);

    if let None = create_thread_id {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user = find_user.unwrap();

    let find_thread = DB::get(COLLECTION_THREAD)
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
        ])
        .filter(vec![
            is("_id", &create_thread_id),
            is("created_by_id", &user.id),
        ])
        .get_one::<ThreadDTO>(&state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    let mut thread = find_thread.unwrap();

    let mut deleted_attachment: Vec<ThreadAttachment> = Vec::new();
    let mut new_attachment: Vec<ThreadAttachment> = Vec::new();

    let active_attachment = thread.attachment.clone();

    for attachment in active_attachment {
        if let Some(id) = attachment.id {
            if body
                .removed_attachment
                .clone()
                .iter()
                .any(|v| v.contains(&id.to_string()))
            {
                deleted_attachment.push(attachment.into());
            } else {
                new_attachment.push(attachment.into());
            }
        }
    }

    let find_reserved_attachment = DB::get(COLLECTION_RESERVE_ATTACHMENT)
        .filter(vec![is_in("_id", body.new_attachment.clone())])
        .get_all::<ThreadAttachment>(&state.db)
        .await
        .unwrap_or(Vec::new());

    for attachment in find_reserved_attachment {
        deleted_attachment.push(attachment.clone());
        new_attachment.push(attachment);
    }

    let quote = body.quote_thread_id.map_or_else(
        || None,
        |id| ObjectId::from_str(id.as_str()).map_or_else(|_| None, |v| Some(v)),
    );

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let mut insert_thread = DB::update(COLLECTION_THREAD);

    if quote.is_some() {
        insert_thread = insert_thread.set_value("quote_thread_id", &quote.unwrap());
        thread.quote_thread_id = quote;
        thread.quote_thread = None;
    } else {
        insert_thread = insert_thread.set_value("quote_thread_id", None::<i32>);
        thread.quote_thread_id = None;
    }

    if let Some(slug) = body.slug {
        insert_thread = insert_thread.set_value("slug", slug.clone().as_str());
        thread.slug = slug;
    }
    if let Some(title) = body.title {
        insert_thread = insert_thread.set_value("title", title.clone().as_str());
        thread.content = title.clone();
    }
    if let Some(content) = body.content {
        insert_thread = insert_thread.set_value("content", content.clone().as_str());
        thread.content = content.clone();
    }

    if let Some(topics) = body.topics {
        insert_thread = insert_thread.set_value("topics", topics.clone());
        thread.topics = Some(topics);
    }

    let update_thread = insert_thread
        .filter(vec![is("_id", &thread.id)])
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = update_thread {
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let delete_all_reserve_attachment = DB::delete(COLLECTION_RESERVE_ATTACHMENT)
        .filter(vec![is_in(
            "_id",
            deleted_attachment
                .clone()
                .iter()
                .map(|v| v.id.unwrap().to_string())
                .collect::<Vec<String>>(),
        )])
        .many_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = delete_all_reserve_attachment {
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    for deleted in deleted_attachment {
        let _delete_from_bucket = MinIO::new()
            .delete_file(deleted.file_name, BUCKET_THREAD.to_string())
            .await;
    }

    let commit = session.commit_transaction().await;
    if let Err(err) = commit {
        info!(target:"stock::update","insert failed {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    ApiResponse::ok(thread, &i18n.translate("thread::create::success"))
}

pub async fn add_top_answer() -> ApiResponse<ThreadDTO> {
    ApiResponse::failed("")
}

pub async fn delete_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
) -> ApiResponse<ThreadDTO> {
    let i18n = i18n!("user", lang);

    let create_thread_id = create_object_id_option(&thread_id);

    if let None = create_thread_id {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![is("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user = find_user.unwrap();

    let find_thread = DB::get(COLLECTION_THREAD)
        .lookup(&[
            one("user", "created_by_id", "_id", "created_by"),
            one("thread", "quote_thread_id", "_id", "quote_thread"),
            one_merge_to(
                "user",
                "quote_thread.created_by_id",
                "_id",
                "created_by",
                "quote_thread",
            ),
            one("thread", "reply_to_thread_id", "_id", "reply_to_thread"),
            one_merge_to(
                "user",
                "reply_to_thread.created_by_id",
                "_id",
                "created_by",
                "reply_to_thread",
            ),
        ])
        .filter(vec![
            is("_id", &create_thread_id),
            is("created_by_id", &user.id),
        ])
        .get_one::<ThreadDTO>(&state.db)
        .await;
    if let Err(err) = find_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    let thread = find_thread.unwrap();

    let delete_thread = DB::delete(COLLECTION_THREAD)
        .filter(vec![is_in("_id", thread_id)])
        .one(&state.db)
        .await;
    if let Err(err) = delete_thread {
        info!(target:"thread::list","failed to fetch {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    ApiResponse::ok(thread, "message")
}

pub async fn upvote(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let create_thread_id = create_object_id_option(&thread_id);

    if let None = create_thread_id {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let user_email: Option<&String> = auth_context.session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let create_thread_id = ObjectId::from_str(&thread_id);
    if let Err(err) = create_thread_id {
        info!(target:"user::profile::failed"," {:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let create_thread_id = create_thread_id.unwrap();

    let user = find_user.unwrap();

    let find_thread = DB::get(COLLECTION_THREAD_VOTE)
        .filter(vec![
            equal("kind", KIND_UP_VOTE_THREAD),
            is("created_by_id", &user.id),
            is("thread_id", &create_thread_id),
        ])
        .get_one::<ThreadVote>(&state.db)
        .await;

    if let Ok(vote) = find_thread {
        info!(target:"user::profile::failed","already_vote {:?}",vote);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let vote = ThreadVote {
        id: Some(ObjectId::new()),
        created_by_id: user.id,
        thread_id: Some(create_thread_id),
        kind: KIND_UP_VOTE_THREAD.to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let insert_vote = DB::insert(COLLECTION_THREAD_VOTE)
        .one_with_session(vote, &state.db, &mut session)
        .await;
    if let Err(err) = insert_vote {
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let update_counter = DB::update("thread")
        .inc(doc! {
            "up_vote_count":1
        })
        .filter(vec![equal("thread_id", &create_thread_id)])
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = update_counter {
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;

    ApiResponse::failed("")
}

pub async fn down_vote(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let create_thread_id = create_object_id_option(&thread_id);

    if let None = create_thread_id {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let user_email: Option<&String> = auth_context.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &user_email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"user::profile::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }

    let create_thread_id = ObjectId::from_str(&thread_id);
    if let Err(err) = create_thread_id {
        info!(target:"user::profile::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let create_thread_id = create_thread_id.unwrap();

    let user = find_user.unwrap();

    let find_thread = DB::get(COLLECTION_THREAD_VOTE)
        .filter(vec![
            equal("kind", KIND_DOWN_VOTE_THREAD),
            is("created_by_id", &user.id),
            is("thread_id", &create_thread_id),
        ])
        .get_one::<ThreadVote>(&state.db)
        .await;

    if let Err(err) = find_thread {
        info!(target:"user::profile::failed","not vote yet {:?}",err);
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let vote = find_thread.unwrap();
    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let delete_vote = DB::delete(COLLECTION_THREAD_VOTE)
        .filter(vec![equal("_id", &vote.id)])
        .one_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = delete_vote {
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let update_counter = DB::update(COLLECTION_THREAD)
        .inc(doc! {
            "up_vote_count":-1
        })
        .filter(vec![equal("thread_id", &create_thread_id)])
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(err) = update_counter {
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;

    ApiResponse::failed("")
}
