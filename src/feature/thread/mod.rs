use std::str::FromStr;

use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::constant::{
    BUCKET_THREAD, KIND_DISCUSSION, KIND_PUBLIC, KIND_THREAD_ATTACHMENT, KIND_UPVOTE_THREAD,
    REDIS_KEY_USER_EMAIL,
};
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::orm::orm::Orm;
use crate::common::utils::create_object_id_option;
use crate::dto::thread_attachment_dto::ThreadAttachmentDTO;
use crate::dto::thread_dto::ThreadDTO;

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

    let mut find_thread = Orm::get("thread");

    if query.order.is_some() {
        let ordering = query.order.unwrap_or("".to_string());
        if !ordering.is_empty() {
            let split = ordering.split(":").collect::<Vec<&str>>();
            let column = split.get(0);
            let group = split.get(1);
            if column.is_some() {
                if group.is_some() {
                    if split[1] == "ASC" {
                        find_thread = find_thread.group_by_asc(split[0]);
                    } else {
                        find_thread = find_thread.group_by_desc(split[0]);
                    }
                } else {
                    find_thread = find_thread.group_by_desc(split[0]);
                }
            }
        }
    } else {
        find_thread = find_thread.group_by_desc("created_at");
    }

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or("".to_string());
        find_thread = find_thread
            .text()
            .filter_string("$search", None, text.as_str());
    }

    let find_thread = find_thread
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_null("reply_to_thread_id", Some("$eq"))
        .filter_string("kind", Some("$eq"), KIND_PUBLIC)
        .pageable::<ThreadDTO>(page, size, &state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }

    ApiResponse::ok(find_thread.unwrap(), &i18n.translate("thread.list"))
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

    let mut find_thread = Orm::get("thread");

    if query.order.is_some() {
        let ordering = query.order.unwrap_or("".to_string());
        if !ordering.is_empty() {
            let split = ordering.split(":").collect::<Vec<&str>>();
            let column = split.get(0);
            let group = split.get(1);
            if column.is_some() {
                if group.is_some() {
                    if split[1] == "ASC" {
                        find_thread = find_thread.group_by_asc(split[0]);
                    } else {
                        find_thread = find_thread.group_by_desc(split[0]);
                    }
                } else {
                    find_thread = find_thread.group_by_desc(split[0]);
                }
            }
        }
    } else {
        find_thread = find_thread.group_by_desc("created_at");
    }

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or("".to_string());
        find_thread = find_thread
            .text()
            .filter_string("$search", None, text.as_str());
    }

    let find_thread = find_thread
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_null("reply_to_thread_id", Some("$eq"))
        .filter_string("kind", Some("$eq"), KIND_DISCUSSION)
        .pageable::<ThreadDTO>(page, size, &state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }

    ApiResponse::ok(find_thread.unwrap(), &i18n.translate("thread.list"))
}

pub async fn get_list_user_thread(
    state: State<AppState>,
    lang: Lang,
    _auth_context: AuthContext,
    Path(user_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let create_user_id = ObjectId::from_str(&user_id);
    if create_user_id.is_err() {
        info!(target:"","");
        return ApiResponse::not_found(&i18n.translate("thread.list.user.not-found"));
    }

    let mut find_thread = Orm::get("thread");

    if query.order.is_some() {
        let ordering = query.order.unwrap_or("".to_string());
        if !ordering.is_empty() {
            let split = ordering.split(":").collect::<Vec<&str>>();
            let column = split.get(0);
            let group = split.get(1);
            if column.is_some() {
                if group.is_some() {
                    if split[1] == "ASC" {
                        find_thread = find_thread.group_by_asc(split[0]);
                    } else {
                        find_thread = find_thread.group_by_desc(split[0]);
                    }
                } else {
                    find_thread = find_thread.group_by_desc(split[0]);
                }
            }
        }
    } else {
        find_thread = find_thread.group_by_desc("created_at");
    }

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or("".to_string());
        find_thread = find_thread
            .text()
            .filter_string("$search", None, text.as_str());
    }

    let find_thread = find_thread
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_object_id("created_by_id", &create_user_id.unwrap())
        .pageable::<ThreadDTO>(page, size, &state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }

    ApiResponse::ok(find_thread.unwrap(), &i18n.translate("thread.list"))
}

pub async fn get_list_comment_thread(
    state: State<AppState>,
    lang: Lang,
    _auth_context: AuthContext,
    Path(thread_id): Path<String>,
    Query(query): Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadDTO>> {
    let i18n = i18n!("thread", lang);
    //getting connection from pool

    let page = query.clone().page.unwrap_or(0);
    let size = query.clone().size.unwrap_or(10);

    let created_thread_id = ObjectId::from_str(&thread_id);
    if created_thread_id.is_err() {
        return ApiResponse::not_found(&i18n.translate("thread not found"));
    }
    let created_thread_id = created_thread_id.unwrap();

    let mut find_thread = Orm::get("thread");

    if query.order.is_some() {
        let ordering = query.order.unwrap_or("".to_string());
        if !ordering.is_empty() {
            let split = ordering.split(":").collect::<Vec<&str>>();
            let column = split.get(0);
            let group = split.get(1);
            if column.is_some() {
                if group.is_some() {
                    if split[1] == "ASC" {
                        find_thread = find_thread.group_by_asc(split[0]);
                    } else {
                        find_thread = find_thread.group_by_desc(split[0]);
                    }
                } else {
                    find_thread = find_thread.group_by_desc(split[0]);
                }
            }
        }
    } else {
        find_thread = find_thread.group_by_desc("created_at");
    }

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or("".to_string());
        find_thread = find_thread
            .text()
            .filter_string("$search", None, text.as_str());
    }

    let find_thread = find_thread
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_object_id("reply_to_thread_id", &created_thread_id)
        .filter_string("kind", Some("$eq"), KIND_PUBLIC)
        .pageable::<ThreadDTO>(page, size, &state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }

    ApiResponse::ok(find_thread.unwrap(), &i18n.translate("thread.list"))
}

pub async fn upload_attachment(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    form_data: SingleFileExtractor,
) -> ApiResponse<ThreadAttachmentDTO> {
    let i18n = i18n!("user", lang);

    let validate = form_data.validate();
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
    if session.is_err() {
        let err = session.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;
    let save_attachment = Orm::insert("reserve-attachment")
        .one_with_session(attachment.clone(), &state.db, &mut session)
        .await;
    if save_attachment.is_err() {
        info!(target:"attachment::upload::faile","{:?}",save_attachment.unwrap_err());
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

    if upload.is_err() {
        info!(target:"attachment::upload::faile","{:?}",upload.unwrap_err());
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

    let user = find_user.unwrap();

    let find_all_attachment = Orm::get("reserve-attachment")
        .filter_array("_id", Some("$in"), body.attachment.clone())
        .all::<ThreadAttachment>(&state.db)
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
        kind: KIND_PUBLIC.to_string(),
        slug: body.slug,
        title: body.title,
        content: body.content,
        attachment: find_all_attachment.clone(),
        up_vote_count: 0,
        down_vote_count: 0,
        quote_count: 0,
        reply_count: 0,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    let session = state.db.start_session().await;
    if session.is_err() {
        let err = session.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let insert_thread = Orm::insert("thread")
        .one_with_session(thread, &state.db, &mut session)
        .await;

    if insert_thread.is_err() {
        let err = insert_thread.unwrap_err();
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    //update counter
    if quote.is_some() {
        let update_counter = Orm::update("thread")
            .inc(doc! {
                "quote_count":1
            })
            .filter_object_id("_id", &quote.unwrap())
            .execute_one_with_session(&state.db, &mut session)
            .await;
        if update_counter.is_err() {
            let err = update_counter.unwrap_err();
            info!(target:"stock::update","insert failed {:?}",err);
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("thread::create::failed"));
        }
    }

    if reply.is_some() {
        let update_counter = Orm::update("thread")
            .inc(doc! {
                "reply_count":1
            })
            .filter_object_id("_id", &quote.unwrap())
            .execute_one_with_session(&state.db, &mut session)
            .await;
        if update_counter.is_err() {
            let err = update_counter.unwrap_err();
            info!(target:"stock::update","insert failed {:?}",err);
            let _ = session.abort_transaction().await;
            return ApiResponse::failed(&i18n.translate("thread::create::failed"));
        }
    }
    //end update counter

    let delete_all_reserve_attachment = Orm::delete("reserve-attachment")
        .filter_array("_id", Some("$in"), body.attachment)
        .many_with_session(&state.db, &mut session)
        .await;

    if delete_all_reserve_attachment.is_err() {
        let err = delete_all_reserve_attachment.unwrap_err();
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
    if commit.is_err() {
        let err = commit.unwrap_err();
        info!(target:"stock::update","insert failed {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let find_thread = Orm::get("thread")
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .filter_object_id("_id", &insert_thread.unwrap())
        .one::<ThreadDTO>(&state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
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
    if validate.is_err() {
        let err = validate.unwrap_err();
        info!(target:"user::profile::validation-error","{:?}",err.clone());
        return ApiResponse::error_validation(err, i18n.translate("user.profile.failed").as_str());
    }

    let create_thread_id = create_object_id_option(&thread_id);

    if create_thread_id.is_none() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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

    let user = find_user.unwrap();

    let find_thread = Orm::get("thread")
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_object_id("_id", &create_thread_id.unwrap())
        .filter_object_id("created_by_id", &user.id.unwrap())
        .one::<ThreadDTO>(&state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    let mut thread = find_thread.unwrap();

    let mut deleted_attachement: Vec<ThreadAttachment> = Vec::new();
    let mut new_attachement: Vec<ThreadAttachment> = Vec::new();

    let active_attachment = thread.attachment.clone();

    for attachment in active_attachment {
        if let Some(id) = attachment.id {
            if body
                .removed_attachment
                .clone()
                .iter()
                .any(|v| v.contains(&id.to_string()))
            {
                deleted_attachement.push(attachment.into());
            } else {
                new_attachement.push(attachment.into());
            }
        }
    }

    let find_reserved_attachement = Orm::get("reserve-attachment")
        .filter_array("_id", Some("$in"), body.new_attachment.clone())
        .all::<ThreadAttachment>(&state.db)
        .await
        .unwrap_or(Vec::new());

    for attachment in find_reserved_attachement {
        deleted_attachement.push(attachment.clone());
        new_attachement.push(attachment);
    }

    let quote = body.quote_thread_id.map_or_else(
        || None,
        |id| ObjectId::from_str(id.as_str()).map_or_else(|_| None, |v| Some(v)),
    );

    let session = state.db.start_session().await;
    if session.is_err() {
        let err = session.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let mut insert_thread = Orm::update("thread");

    if quote.is_some() {
        insert_thread = insert_thread.set_object("quote_thread_id", &quote.unwrap());
        thread.quote_thread_id = quote;
        thread.quote_thread = None;
    } else {
        insert_thread = insert_thread.set_null("quote_thread_id");
        thread.quote_thread_id = None;
    }

    if body.slug.is_some() {
        insert_thread =
            insert_thread.set_str("slug", body.slug.clone().unwrap_or("".to_string()).as_str());
        thread.slug = body.slug.unwrap_or("".to_string());
    }
    if body.title.is_some() {
        insert_thread = insert_thread.set_str(
            "title",
            body.title.clone().unwrap_or("".to_string()).as_str(),
        );
        thread.content = body.title.unwrap_or("".to_string());
    }
    if body.content.is_some() {
        insert_thread = insert_thread.set_str(
            "content",
            body.content.clone().unwrap_or("".to_string()).as_str(),
        );
        thread.content = body.content.unwrap_or("".to_string());
    }

    let update_thread = insert_thread
        .filter_object_id("_id", &thread.id.unwrap())
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if update_thread.is_err() {
        let err = update_thread.unwrap_err();
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let delete_all_reserve_attachment = Orm::delete("reserve-attachment")
        .filter_array(
            "_id",
            Some("$in"),
            deleted_attachement
                .clone()
                .iter()
                .map(|v| v.id.unwrap().to_string())
                .collect(),
        )
        .many_with_session(&state.db, &mut session)
        .await;

    if delete_all_reserve_attachment.is_err() {
        let err = delete_all_reserve_attachment.unwrap_err();
        info!(target:"stock::update","insert failed {:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    for deleted in deleted_attachement {
        let _delete_from_bucket = MinIO::new()
            .delete_file(deleted.file_name, BUCKET_THREAD.to_string())
            .await;
    }

    let commit = session.commit_transaction().await;
    if commit.is_err() {
        let err = commit.unwrap_err();
        info!(target:"stock::update","insert failed {:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    ApiResponse::ok(thread, &i18n.translate("thread::create::success"))
}

pub async fn delete_thread(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
) -> ApiResponse<ThreadDTO> {
    let i18n = i18n!("user", lang);

    let create_thread_id = create_object_id_option(&thread_id);

    if create_thread_id.is_none() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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

    let user = find_user.unwrap();

    let find_thread = Orm::get("thread")
        .join_one("user", "created_by_id", "_id", "created_by")
        .join_one("thread", "quote_thread_id", "_id", "quote_thread")
        .join_one_nested(
            "user",
            "quote_thread.created_by_id",
            "_id",
            "created_by",
            "quote_thread",
        )
        .join_one("thread", "reply_to_thread_id", "_id", "reply_to_thread")
        .join_one_nested(
            "user",
            "reply_to_thread.created_by_id",
            "_id",
            "created_by",
            "reply_to_thread",
        )
        .and()
        .filter_object_id("_id", &create_thread_id.unwrap())
        .filter_object_id("created_by_id", &user.id.unwrap())
        .one::<ThreadDTO>(&state.db)
        .await;
    if find_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",find_thread.unwrap_err());
        return ApiResponse::failed(&i18n.translate("thread.not-found"));
    }
    let thread = find_thread.unwrap();

    let delete_thread = Orm::delete("thread")
        .filter_object_id("_id", &create_thread_id.unwrap())
        .one(&state.db)
        .await;
    if delete_thread.is_err() {
        info!(target:"thread::list","failed to fetch {:?}",delete_thread.unwrap_err());
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

    if create_thread_id.is_none() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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

    let create_thread_id = ObjectId::from_str(&thread_id);
    if create_thread_id.is_err() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let create_thread_id = create_thread_id.unwrap();

    let user = find_user.unwrap();

    let find_thread = Orm::get("thread-vote")
        .and()
        .filter_string("kind", Some("$eq"), KIND_UPVOTE_THREAD)
        .filter_object_id("created_by_id", &user.id.unwrap())
        .filter_object_id("thread_id", &create_thread_id)
        .one::<ThreadVote>(&state.db)
        .await;

    if find_thread.is_ok() {
        info!(target:"user::profile::failed","already_vote");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let session = state.db.start_session().await;
    if session.is_err() {
        let err = session.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let vote = ThreadVote {
        id: Some(ObjectId::new()),
        created_by_id: user.id,
        thread_id: Some(create_thread_id),
        kind: KIND_UPVOTE_THREAD.to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let insert_vote = Orm::insert("thread-vote")
        .one_with_session(vote, &state.db, &mut session)
        .await;
    if insert_vote.is_err() {
        let err = insert_vote.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let update_counter = Orm::update("thread")
        .inc(doc! {
            "up_vote_count":1
        })
        .filter_object_id("_id", &create_thread_id)
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if update_counter.is_err() {
        let err = update_counter.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;

    ApiResponse::failed("")
}

pub async fn downvote(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(thread_id): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("user", lang);

    let create_thread_id = create_object_id_option(&thread_id);

    if create_thread_id.is_none() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
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

    let create_thread_id = ObjectId::from_str(&thread_id);
    if create_thread_id.is_err() {
        info!(target:"user::profile::failed","connection error");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let create_thread_id = create_thread_id.unwrap();

    let user = find_user.unwrap();

    let find_thread = Orm::get("thread-vote")
        .and()
        .filter_string("kind", Some("$eq"), KIND_UPVOTE_THREAD)
        .filter_object_id("created_by_id", &user.id.unwrap())
        .filter_object_id("thread_id", &create_thread_id)
        .one::<ThreadVote>(&state.db)
        .await;

    if find_thread.is_err() {
        info!(target:"user::profile::failed","not vote yet");
        return ApiResponse::failed(i18n.translate("user.profile.failed").as_str());
    }
    let vote = find_thread.unwrap();
    let session = state.db.start_session().await;
    if session.is_err() {
        let err = session.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let delete_vote = Orm::delete("thread-vote")
        .filter_object_id("_id", &vote.id.unwrap())
        .one_with_session(&state.db, &mut session)
        .await;

    if delete_vote.is_err() {
        let err = delete_vote.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }
    let update_counter = Orm::update("thread")
        .inc(doc! {
            "up_vote_count":-1
        })
        .filter_object_id("_id", &create_thread_id)
        .execute_one_with_session(&state.db, &mut session)
        .await;

    if update_counter.is_err() {
        let err = update_counter.unwrap_err();
        info!(target:"stock::update","{:?}",err);
        let _ = session.abort_transaction().await;
        return ApiResponse::failed(&i18n.translate("thread::create::failed"));
    }

    let _commit = session.commit_transaction().await;

    ApiResponse::failed("")
}
