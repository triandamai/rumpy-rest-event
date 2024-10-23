use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::JwtClaims;
use crate::common::sse::sse_builder::{SseBuilder, SseTarget};
use crate::entity::thread::{Thread, ThreadDTO, ThreadWithDetailDTO};
use crate::entity::thread_comment::{ThreadComment, ThreadCommentDTO, ThreadCommentDetailDTO};
use crate::entity::thread_vote::{ThreadVote, ThreadVoteDTO, VoteType};
use crate::feature::auth::auth_model::USER_ID_KEY;
use crate::feature::threads::threads_model::{CreateCommentRequest, CreateNewThreadRequest, CreateVoteRequest, FilterThreadRequest};
use axum::extract::{Path, Query, State};
use axum::Json;
use bson::oid::Error;
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime};
use serde_json::json;
use std::str::FromStr;
use validator::Validate;

pub async fn get_list_threads_by_current_user(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadWithDetailDTO>> {
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let user_id_as_object = user_id_as_object.unwrap();


    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);
    let mut pipeline = vec![
        doc! {"$match":{"created_by_id":user_id_as_object.clone()}}
    ];
    let mut query_count = vec![doc! {"$match":{"created_by_id":user_id_as_object.clone()}}];

    if query.q.is_some() {
        pipeline.push(doc! {"$text":query.q.clone().unwrap()});
        query_count.push(doc! {"$text":query.q.clone().unwrap()});
    }
    pipeline.push(Thread::create_field_lookup_thread());
    pipeline.push(Thread::create_field_lookup_attachment());
    pipeline.push(Thread::create_field_lookup_user());
    pipeline.push(Thread::create_field_unwind("$created_by".to_string()));
    pipeline.push(Thread::create_field_limit(size));
    pipeline.push(Thread::create_field_skip((page.clone() - 1) * size.clone()));


    let get_list_thread = Thread::find(
        pipeline,
        query_count,
        page,
        size,
        &state.db,
    ).await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}

pub async fn get_list_threads(
    mut state: State<AppState>,
    auth: JwtClaims,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadWithDetailDTO>> {
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }


    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);

    let mut pipeline = vec![];
    let mut query_count = vec![];

    if query.q.is_some() {
        pipeline.push(doc! {"$text":query.q.clone().unwrap()});
        query_count.push(doc! {"$text":query.q.clone().unwrap()});
    }
    pipeline.push(Thread::create_field_lookup_thread());
    pipeline.push(Thread::create_field_lookup_attachment());
    pipeline.push(Thread::create_field_lookup_user());
    pipeline.push(Thread::create_field_unwind("$created_by".to_string()));
    pipeline.push(Thread::create_field_limit(size));
    pipeline.push(Thread::create_field_skip((page.clone() - 1) * size.clone()));


    let get_list_thread = Thread::find(
        pipeline,
        query_count,
        page,
        size,
        &state.db,
    ).await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}

pub async fn get_list_filter_threads(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<FilterThreadRequest>,
) -> ApiResponse<PagingResponse<ThreadWithDetailDTO>> {
    if body.q.clone().is_some() && body.mentions.clone().unwrap_or(vec![]).len() > 0 || body.tags.clone().unwrap_or(vec![]).len() > 0 {
        return ApiResponse::failed("Pencarian hanya boleh salah satu dari \"q\"  atau \"tags\" atau \"mentions\"".to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let page = body.page.unwrap_or(1);
    let size = body.size.unwrap_or(20);

    let mut pipeline = vec![];
    let mut query_count = vec![];

    if body.q.is_some() {
        pipeline.push(doc! {"$match":{"$text":{"$search":body.q.clone().unwrap()}}});
        query_count.push(doc! {"$match":{"$text":{"$search":body.q.clone().unwrap()}}});
    }

    if body.mentions.clone().unwrap_or(vec![]).len() > 0 {
        pipeline.push(doc! {
            "mentions":{"$in":body.mentions.clone().unwrap()}
        });
        query_count.push(doc! {
            "mentions":{"$in":body.mentions.clone().unwrap()}
        });
    }
    if body.tags.clone().unwrap_or(vec![]).len() > 0 {
        pipeline.push(doc! {
            "tags":{"$in":body.tags.clone().unwrap()}
        });
        query_count.push(doc! {
            "tags":{"$in":body.tags.clone().unwrap()}
        });
    }
    pipeline.push(Thread::create_field_lookup_thread());
    pipeline.push(Thread::create_field_lookup_attachment());
    pipeline.push(Thread::create_field_lookup_user());
    pipeline.push(Thread::create_field_unwind("$created_by".to_string()));
    pipeline.push(Thread::create_field_limit(size));
    pipeline.push(Thread::create_field_skip((page.clone() - 1) * size.clone()));


    let get_list_thread = Thread::find(
        pipeline,
        query_count,
        page,
        size,
        &state.db,
    ).await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}


pub async fn get_list_filter_threads_by_current_user(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<FilterThreadRequest>,
) -> ApiResponse<PagingResponse<ThreadWithDetailDTO>> {
    if body.q.clone().is_some() && body.mentions.clone().unwrap_or(vec![]).len() > 0 || body.tags.clone().unwrap_or(vec![]).len() > 0 {
        return ApiResponse::failed("Pencarian hanya boleh salah satu dari \"q\"  atau \"tags\" atau \"mentions\"".to_string());
    }

    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let user_id_as_object = user_id_as_object.unwrap();


    let page = body.page.unwrap_or(1);
    let size = body.size.unwrap_or(20);

    let mut pipeline = vec![];
    let mut query_count = vec![];

    if body.q.is_some() {
        pipeline.push(doc! {"$match":{"$text":{"$search":body.q.clone().unwrap()}}});
        query_count.push(doc! {"$match":{"$text":{"$search":body.q.clone().unwrap()}}});
    }

    pipeline.push(doc! {"$match":{"created_by_id":user_id_as_object.clone()}});
    query_count.push(doc! {"$match":{"created_by_id":user_id_as_object.clone()}});

    if body.mentions.clone().unwrap_or(vec![]).len() > 0 {
        pipeline.push(doc! {
            "mentions":{"$in":body.mentions.clone().unwrap()}
        });
        query_count.push(doc! {
            "mentions":{"$in":body.mentions.clone().unwrap()}
        });
    }
    if body.tags.clone().unwrap_or(vec![]).len() > 0 {
        pipeline.push(doc! {
            "tags":{"$in":body.tags.clone().unwrap()}
        });
        query_count.push(doc! {
            "tags":{"$in":body.tags.clone().unwrap()}
        });
    }
    pipeline.push(Thread::create_field_lookup_thread());
    pipeline.push(Thread::create_field_lookup_attachment());
    pipeline.push(Thread::create_field_lookup_user());
    pipeline.push(Thread::create_field_unwind("$created_by".to_string()));
    pipeline.push(Thread::create_field_limit(size));
    pipeline.push(Thread::create_field_skip((page.clone() - 1) * size.clone()));


    info!(target: "filter-current-user","{:?}",body.q);
    info!(target: "filter-current-user","{:?}",pipeline.clone());
    let get_list_thread = Thread::find(
        pipeline,
        query_count,
        page,
        size,
        &state.db,
    ).await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}

pub async fn get_detail_thread(
    mut state: State<AppState>,
    Path(thread_id): Path<String>,
    auth: JwtClaims,
) -> ApiResponse<ThreadWithDetailDTO> {
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let thread_id = ObjectId::from_str(thread_id.as_str());
    if thread_id.is_err() {
        return ApiResponse::not_found("Gagal menemukan thread".to_string());
    }
    let pipeline = vec![
        doc! {"$match":{"_id":thread_id.unwrap()}},
        Thread::create_field_lookup_thread(),
        Thread::create_field_lookup_attachment(),
        Thread::create_field_lookup_user(),
        Thread::create_field_unwind("$created_by".to_string()),
        Thread::create_field_limit(1),
    ];

    let find = Thread::find_one(pipeline, &state.db).await;
    info!(target:"get_detail_thread", "Found {:?} thread", find.clone());
    if find.is_none() {
        return ApiResponse::not_found("Data tidak ditemukan".to_string());
    }
    ApiResponse::ok(find.unwrap(), "Data postingan anda.")
}

pub async fn create_new_thread(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateNewThreadRequest>,
) -> ApiResponse<ThreadDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }

    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let user_id_as_object = user_id_as_object.unwrap();

    let quote_thread = match ObjectId::from_str(body.quote_thread_id.clone().unwrap_or(String::from("n/a")).as_str()) {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let current_time = DateTime::now();
    let mut create_thread = Thread {
        id: None,
        created_by_id: Some(user_id_as_object),
        quote_thread_id: quote_thread,
        title: body.title.clone(),
        content: body.content.clone(),
        watch_count: 0,
        comment_count: 0,
        up_vote_count: 0,
        down_vote_count: 0,
        mentions: body.mentions.clone(),
        tags: body.tags.clone(),
        created_at: current_time.clone(),
        updated_at: current_time.clone(),
    };
    let saved = create_thread
        .save(&state.db)
        .await;

    if saved.is_none() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let saved = saved.unwrap();
    ApiResponse::ok(saved.to_dto(), "Berhasil membuat thread")
}

pub async fn send_thread_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateVoteRequest>,
) -> ApiResponse<ThreadVoteDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::failed("Failed".to_string());
    }
    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .map_or_else(|| String::from("n/a"), |value| value.clone());
    let user_id = ObjectId::from_str(user_id.as_str());
    if user_id.is_err() {
        return ApiResponse::not_found("Gagal membuat user".to_string());
    }
    let user_id = user_id.unwrap();


    let ref_id = match ObjectId::from_str(body.ref_id.clone().as_str()) {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let find_thread = Thread::find_one_basic(
        vec![
            doc! {"$match":{"_id":ref_id.clone()}},
            Thread::create_field_limit(1)
        ],
        &state.db,
    ).await;

    if find_thread.is_none() {
        return ApiResponse::not_found("Tidak menemukan postingan".to_string());
    }
    let find_thread = find_thread.unwrap();

    let mut vote = ThreadVote {
        id: None,
        ref_id: ref_id.clone(),
        vote_by: Some(user_id),
        vote_type: body.vote_type.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let saved = vote
        .save(&state.db)
        .await;
    if saved.is_none() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }

    let saved = saved.unwrap();
    let doc = if body.vote_type.clone().eq("UpVote") {
        doc! {"$inc":{"up_vote_count":1}}
    } else {
        doc! {"$inc":{"down_vote_count":1}}
    };

    let _update_vote = Thread::update_one(
        doc! {"_id": ref_id.clone()},
        doc,
        &state.db,
    ).await;


    if (body.vote_type.eq("UpVote")) {
        if let Some(id) = find_thread.created_by_id {
            let _sse = state
                .sse
                .send(
                    SseBuilder::new(
                        SseTarget::create()
                            .set_user_id(id.to_string())
                            .set_event_name("thread-upvote".to_string()),
                        json!({"message":"Seseorang menyukai postingan Anda."}),
                    )
                ).await;
        };
    }

    ApiResponse::ok(saved.to_dto(), "Berhasil vote")
}

pub async fn undo_thread_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateVoteRequest>,
) -> ApiResponse<ThreadVoteDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::failed("Failed".to_string());
    }
    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .map_or_else(|| String::from("n/a"), |value| value.clone());
    let user_id = ObjectId::from_str(user_id.as_str());
    if user_id.is_err() {
        return ApiResponse::not_found("Gagal membuat user".to_string());
    }
    let user_id = user_id.unwrap();
    let ref_id = match ObjectId::from_str(body.ref_id.as_str()) {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let mut vote = ThreadVote {
        id: None,
        ref_id: ref_id.clone(),
        vote_by: Some(user_id),
        vote_type: body.vote_type.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let saved = vote
        .delete(&state.db)
        .await;
    if saved.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }

    let saved = saved.unwrap();
    let doc = if body.vote_type.clone().eq("UpVote") {
        doc! {"$dec":{"up_vote_count":1}}
    } else {
        doc! {"$dec":{"down_vote_count":1}}
    };

    let _update_vote = Thread::update_one(
        doc! {"_id": ref_id},
        doc,
        &state.db,
    ).await;

    ApiResponse::ok(saved.to_dto(), "Berhasil vote")
}


pub async fn undo_comment_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateVoteRequest>,
) -> ApiResponse<ThreadVoteDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::failed("Failed".to_string());
    }
    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .map_or_else(|| String::from("n/a"), |value| value.clone());
    let user_id = ObjectId::from_str(user_id.as_str());
    if user_id.is_err() {
        return ApiResponse::not_found("Gagal membuat user".to_string());
    }
    let user_id = user_id.unwrap();
    let ref_id = match ObjectId::from_str(body.ref_id.as_str()) {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let mut vote = ThreadVote {
        id: None,
        ref_id: ref_id.clone(),
        vote_by: Some(user_id),
        vote_type: body.vote_type.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let saved = vote
        .delete(&state.db)
        .await;
    if saved.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }

    let saved = saved.unwrap();
    let doc = if body.vote_type.clone().eq("UpVote") {
        doc! { "$dec":{  "up_vote_count":1 }}
    } else {
        doc! {"$dec":{"down_vote_count":1}}
    };

    let _update_vote = ThreadComment::update_one(
        doc! {"_id": ref_id},
        doc,
        &state.db,
    ).await;


    ApiResponse::ok(saved.to_dto(), "Berhasil vote")
}

pub async fn send_comment_vote(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateVoteRequest>,
) -> ApiResponse<ThreadVoteDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::failed("Failed".to_string());
    }
    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .map_or_else(|| String::from("n/a"), |value| value.clone());
    let user_id = ObjectId::from_str(user_id.as_str());
    if user_id.is_err() {
        return ApiResponse::not_found("Gagal membuat user".to_string());
    }
    let user_id = user_id.unwrap();
    let ref_id = match ObjectId::from_str(body.ref_id.as_str()) {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let find_comment = ThreadComment::find_one(
        doc! {
            "_id": ref_id.clone()
        },
        &state.db,
    ).await;
    if find_comment.is_none() {
        return ApiResponse::failed("Gagal vote.".to_string());
    }
    let find_comment = find_comment.unwrap();


    let mut vote = ThreadVote {
        id: None,
        ref_id: ref_id.clone(),
        vote_by: Some(user_id),
        vote_type: body.vote_type.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let saved = vote
        .save(&state.db)
        .await;
    if saved.is_none() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }

    let saved = saved.unwrap();
    let doc = if body.vote_type.clone().eq("UpVote") {
        doc! { "$inc":{  "up_vote_count":1 }}
    } else {
        doc! {"$inc":{"down_vote_count":1}}
    };

    let _update_vote = ThreadComment::update_one(
        doc! {"_id": ref_id},
        doc,
        &state.db,
    ).await;

    if (body.vote_type.eq("UpVote")) {
        if let Some(id) = find_comment.created_by_id {
            let _sse = state
                .sse
                .send(
                    SseBuilder::new(
                        SseTarget::create()
                            .set_user_id(id.to_string())
                            .set_event_name("comment-upvote".to_string()),
                        json!({"message":"Seseorang menyukai komentar Anda"}),
                    )
                ).await;
        };
    }
    ApiResponse::ok(saved.to_dto(), "Berhasil vote")
}

pub async fn get_list_comment_by_thread(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(thread_id): Path<String>,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadCommentDetailDTO>> {
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let thread_id = ObjectId::from_str(thread_id.as_str());
    if thread_id.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let thread_id = thread_id.unwrap();


    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);

    let get_list_thread = ThreadComment::find_all_paging(
        Some(doc! {
            "$match":{
                "thread_id":thread_id,
                 "comment_id": { "$exists": false }  // Check if 'related_id' is missing
            }
        }), page, size, &state.db)
        .await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}

pub async fn get_list_reply_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    Path(comment_id): Path<String>,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ThreadCommentDetailDTO>> {
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());
    if get_session.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }

    let comment_id = ObjectId::from_str(comment_id.as_str());
    if comment_id.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let comment_id = comment_id.unwrap();


    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);

    let get_list_thread = ThreadComment::find_all_paging(
        Some(doc! {
            "$match":{
                "comment_id":{
                    "$exists": true,   // Check if 'related_id' exists
                    "$ne": null,       // Ensure 'related_id' is not null
                    "$eq": comment_id // Ensure 'related_id' equals a specific ObjectId
                }
            }
        }), page, size, &state.db)
        .await;

    ApiResponse::ok(get_list_thread, "Data postingan anda.")
}

pub async fn create_comment(
    mut state: State<AppState>,
    auth: JwtClaims,
    body: Json<CreateCommentRequest>,
) -> ApiResponse<ThreadCommentDTO> {
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }
    let get_session = state
        .redis
        .get_session_sign_in(auth.sub.clone().as_str());

    if get_session.is_err() {
        return ApiResponse::failed("".to_string());
    }
    let get_session = get_session.unwrap();
    let user_id = get_session
        .get(USER_ID_KEY)
        .unwrap_or(&String::from("n/a"))
        .clone();
    let user_id_as_object = ObjectId::from_str(user_id.as_str());

    if user_id_as_object.is_err() {
        return ApiResponse::un_authorized("Anda tidak memiliki akses.");
    }
    let user_id_as_object = user_id_as_object.unwrap();

    let current_time = DateTime::now();
    let thread_id = ObjectId::from_str(body.thread_id.as_str());
    if thread_id.is_err() {
        return ApiResponse::not_found("Gagal membuat thread.".to_string());
    }

    let thread_id = thread_id.unwrap();

    let create_comment_id = match body.comment_id.clone() {
        None => None,
        Some(comment_id) => ObjectId::from_str(comment_id.as_str()).map_or_else(|_| None, |v| Some(v))
    };

    let find_thread = Thread::find_one_basic(vec![doc! {"$match":{"_id":thread_id.clone()}}], &state.db)
        .await;

    if find_thread.is_none() {
        return ApiResponse::failed("Postingan tidak ditemukan.".to_string());
    }
    let find_thread = find_thread.unwrap();

    let mut comment = ThreadComment {
        id: None,
        thread_id: Some(thread_id),
        comment_id: create_comment_id,
        created_by_id: Some(user_id_as_object),
        reply_count: 0,
        up_vote_count: 0,
        down_vote_count: 0,
        tags: body.tags.clone(),
        mentions: body.mentions.clone(),
        body: body.content.clone(),
        created_at: current_time.clone(),
        updated_at: current_time.clone(),
    };
    let saved = comment
        .save(&state.db)
        .await;
    if saved.is_none() {
        return ApiResponse::failed("Gagal membuat thread.".to_string());
    }
    let saved = saved.unwrap();
    let _update_thread = Thread::update_one(
        doc! {"_id":thread_id},
        doc! {"$inc":{
                "comment_count":1,
                "watch_count":1
            }},
        &state.db,
    ).await;

    if let Some(created_by) = find_thread.created_by_id {
        let subject = match body.comment_id{
            None => "Mengomentari Postingan".to_string(),
            Some(_) => "Membalas Komentar".to_string(),
        };
        let format = format!("Seseorang {} Anda.",subject);
        let _sse = state
            .sse
            .send(
                SseBuilder::new(
                    SseTarget::create()
                        .set_user_id(created_by.to_string())
                        .set_event_name("comment-upvote".to_string()),
                    json!({"message":format}),
                )
            ).await;
    };

    ApiResponse::ok(saved.to_dto(), "Berhasil membuat comment")
}