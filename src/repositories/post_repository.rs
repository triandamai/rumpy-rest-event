use std::collections::HashMap;

use itertools::Itertools;
use log::info;
use sqlx::{Acquire, Error, Pool, Postgres};

use crate::common::api_response::{Count, PagingResponse};
use crate::entity::post::{Post, PostAttachmentWithFile, PostAttachmentWithFileAndPost, PostComment, PostCommentDownVote, PostCommentUpVote, PostCommentWatch, PostCommentWatchWithUserAndComment, PostCommentWithUserAndPost, PostDownVote, PostUpVote, PostWatch, PostWithUserAndSpace, PostWithUserAndSpaceAndAttachment};

pub async fn get_list_post(
    page: i32,
    size: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<PostWithUserAndSpaceAndAttachment> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM post"#,
    ).fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_space","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, PostWithUserAndSpace>(
        r#"
        SELECT account.*,space,account
        FROM post
        JOIN user_credential as account ON account.id = post.user_id
        JOIN space as space ON space.id = post.space_id
        LIMIT $1 OFFSET $2
        ORDER BY created_at DESC
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target:"get_list_space","{:?}",data.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let posts = data.unwrap();
    let ids: Vec<i32> = posts.clone().iter().map(|v| v.id).collect();

    let find_attachment = sqlx::query_as::<_, PostAttachmentWithFile>(r#"
        SELECT * FROM post_attachment
        WHERE id IN($1)
    "#)
        .bind(ids)
        .fetch_all(pool)
        .await;

    let attachments = match find_attachment {
        Ok(founds) => founds
            .into_iter()
            .into_group_map_by(|v| v.post_id),
        Err(err) => HashMap::new()
    };

    let transform: Vec<PostWithUserAndSpaceAndAttachment> = posts.into_iter()
        .map(|post| {
            let value = post.clone();
            PostWithUserAndSpaceAndAttachment {
                id: value.id,
                user_id: value.user_id,
                space_id: value.space_id,
                body: value.body,
                post_type: value.post_type,
                comments: value.comments,
                watch: value.watch,
                up_vote: value.up_vote,
                down_vote: value.down_vote,
                share_count: value.share_count,
                created_at: value.created_at,
                updated_at: value.updated_at,
                user: value.user,
                space: value.space,
                attachments: attachments.get(&post.id.clone()).unwrap_or(&Vec::new()).clone(),
            }
        })
        .collect();

    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_post","{:?}",total.clone());
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: transform,
    }
}

pub async fn get_list_post_by_current_user(
    page: i32,
    size: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<PostWithUserAndSpaceAndAttachment> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM post WHERE post.user_id=$1"#,
    ).bind(user_id.clone()).fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_space","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, PostWithUserAndSpace>(
        r#"
        SELECT account.*,space,account
        FROM post
        JOIN user_credential as account ON account.id = post.user_id
        JOIN space as space ON space.id = post.space_id
        WHERE post.user_id = $1
        LIMIT $2 OFFSET $3
        ORDER BY created_at DESC
        "#,
    )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target:"get_list_post_by_current_user","{:?}",data.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let posts = data.unwrap();
    let ids: Vec<i32> = posts.clone().iter().map(|v| v.id).collect();

    let find_attachment = sqlx::query_as::<_, PostAttachmentWithFile>(r#"
        SELECT * FROM post_attachment
        WHERE id IN($1)
    "#)
        .bind(ids)
        .fetch_all(pool)
        .await;

    let attachments = match find_attachment {
        Ok(founds) => founds
            .into_iter()
            .into_group_map_by(|v| v.post_id),
        Err(err) => HashMap::new()
    };

    let transform: Vec<PostWithUserAndSpaceAndAttachment> = posts.into_iter()
        .map(|post| {
            let value = post.clone();
            PostWithUserAndSpaceAndAttachment {
                id: value.id,
                user_id: value.user_id,
                space_id: value.space_id,
                body: value.body,
                post_type: value.post_type,
                comments: value.comments,
                watch: value.watch,
                up_vote: value.up_vote,
                down_vote: value.down_vote,
                share_count: value.share_count,
                created_at: value.created_at,
                updated_at: value.updated_at,
                user: value.user,
                space: value.space,
                attachments: attachments.get(&post.id.clone()).unwrap_or(&Vec::new()).clone(),
            }
        })
        .collect();

    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_post_by_current_user","{:?}",total.clone());
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: transform,
    }
}

pub async fn get_list_post_by_space(
    page: i32,
    size: i32,
    space_id: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<PostWithUserAndSpaceAndAttachment> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM post.space_id=$1"#,
    ).bind(space_id.clone()).fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_space","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, PostWithUserAndSpace>(
        r#"
        SELECT account.*,space,account
        FROM post
        JOIN user_credential as account ON account.id = post.user_id
        JOIN space as space ON space.id = post.space_id
        WHERE post.space_id = $1
        LIMIT $2 OFFSET $3
        ORDER BY created_at DESC
        "#,
    )
        .bind(space_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target:"get_list_post_by_current_user","{:?}",data.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let posts = data.unwrap();
    let ids: Vec<i32> = posts.clone().iter().map(|v| v.id).collect();

    let find_attachment = sqlx::query_as::<_, PostAttachmentWithFile>(r#"
        SELECT * FROM post_attachment
        WHERE id IN($1)
    "#)
        .bind(ids)
        .fetch_all(pool)
        .await;

    let attachments = match find_attachment {
        Ok(founds) => founds
            .into_iter()
            .into_group_map_by(|v| v.post_id),
        Err(err) => HashMap::new()
    };

    let transform: Vec<PostWithUserAndSpaceAndAttachment> = posts.into_iter()
        .map(|post| {
            let value = post.clone();
            PostWithUserAndSpaceAndAttachment {
                id: value.id,
                user_id: value.user_id,
                space_id: value.space_id,
                body: value.body,
                post_type: value.post_type,
                comments: value.comments,
                watch: value.watch,
                up_vote: value.up_vote,
                down_vote: value.down_vote,
                share_count: value.share_count,
                created_at: value.created_at,
                updated_at: value.updated_at,
                user: value.user,
                space: value.space,
                attachments: attachments.get(&post.id.clone()).unwrap_or(&Vec::new()).clone(),
            }
        })
        .collect();

    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_post_by_current_user","{:?}",total.clone());
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: transform,
    }
}

pub async fn get_post_by_id(
    id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostWithUserAndSpaceAndAttachment, String> {
    let post = sqlx::query_as::<_, PostWithUserAndSpace>(r#"
        SELECT *,space,user_credential FROM post
        JOIN space ON space.id = post.space_id
        JOIN user_credential ON user_credential.id = post.user_id
        WHERE post.id = $1
    "#)
        .fetch_optional(pool)
        .await;

    if post.is_err() {
        info!(target: "get_post_by_id","{:?}",post.unwrap_err());
        return Err("Failed to retrieve post".to_string());
    }
    let post = post.unwrap();
    if post.is_none() {
        info!(target: "get_post_by_id","not found");
        return Err("Failed to retrieve post".to_string());
    }
    let post = post.unwrap();

    let attachment = sqlx::query_as::<_, PostAttachmentWithFile>(r#"
        SELECT * FROM post_attachment
        JOIN storage ON storage.id = post_attachment.file_id
        WHERE post_attachment.post_id= $1
        ORDER BY created_at ASC
    "#)
        .bind(id)
        .fetch_all(pool)
        .await;


    let attachment = attachment.unwrap_or_else(|_| Vec::new());


    let response = PostWithUserAndSpaceAndAttachment {
        id: post.id,
        user_id: post.user_id,
        space_id: post.space_id,
        body: post.body,
        post_type: post.post_type,
        comments: post.comments,
        watch: post.watch,
        up_vote: post.up_vote,
        down_vote: post.down_vote,
        share_count: post.share_count,
        created_at: post.created_at,
        updated_at: post.updated_at,
        user: post.user,
        space: post.space,
        attachments: attachment,
    };

    Ok(response)
}

pub async fn create_post(
    post: Post,
    pool: &Pool<Postgres>,
) -> Result<Post, String> {
    let save_post = sqlx::query_as::<_, Post>(r#"
        INSERT INTO post(user_id,space_id,body,post_type)
        VALUES($1,$2,$3,$4)
        RETURNING *
    "#)
        .bind(post.user_id)
        .bind(post.space_id)
        .bind(post.body)
        .bind(post.post_type)
        .fetch_one(pool)
        .await;

    if save_post.is_err() {
        info!(target: "create_post","{:?}",save_post.unwrap_err());
        return Err("Gagal membuat post".to_string());
    }

    Ok(save_post.unwrap())
}

pub async fn delete_post(
    post_id: i32,
    pool: &Pool<Postgres>,
) -> Result<i32, String> {
    let save_post = sqlx::query(r#"
        DELETE FROM post WHERE id=$1
    "#)
        .bind(post_id)
        .execute(pool)
        .await;

    if save_post.is_err() {
        info!(target: "create_post","{:?}",save_post.unwrap_err());
        return Err("Gagal membuat post".to_string());
    }
    Ok(post_id)
}

pub async fn send_watch(
    post_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostWatch, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        return Err("".to_string());
    }
    let mut trx = open_trx.unwrap();

    let is_exist = sqlx::query_as::<_, PostWatch>(r#"
        SELECT * FROM post_watch
        WHERE user_id=$1 AND post_id=$2
    "#)
        .bind(user_id)
        .bind(post_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        info!(target: "send_watch","Failed to get data");
        let _ = &trx.rollback().await;
        return Err("Kamu suda like postingan ini".to_string());
    }

    let saved_watch_post = sqlx::query_as::<_, PostWatch>(r#"
        INSERT INTO post_watch(user_id,post_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .bind(user_id)
        .bind(post_id)
        .fetch_one(&mut *trx)
        .await;

    if saved_watch_post.is_err() {
        info!(target: "send_like","failed to save like reason:{:?}",saved_watch_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me like postingan.".to_string());
    }

    let increment_watch_post = sqlx::query_as::<_, Post>(r#"
        UPDATE post
        SET watch=watch+1
        WHERE post.id = $1
    "#)
        .bind(post_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_watch_post.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_watch_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    let _ = &trx.commit().await;

    return Ok(saved_watch_post.unwrap());
}

pub async fn send_up_vote(
    post_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostUpVote, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        return Err("".to_string());
    }
    let mut trx = open_trx.unwrap();

    let is_exist = sqlx::query_as::<_, PostUpVote>(r#"
        SELECT * FROM post_up_vote
        WHERE user_id=$1 AND post_id=$2
    "#)
        .bind(user_id)
        .bind(post_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        return Err("Kamu suda like postingan ini".to_string());
    }

    let saved_up_vote_post = sqlx::query_as::<_, PostUpVote>(r#"
        INSERT INTO post_up_vote(user_id,post_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .bind(user_id)
        .bind(post_id)
        .fetch_one(&mut *trx)
        .await;

    if saved_up_vote_post.is_err() {
        info!(target: "send_like","failed to save like reason:{:?}",saved_up_vote_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me like postingan.".to_string());
    }

    let increment_up_vote_post = sqlx::query_as::<_, Post>(r#"
        UPDATE post
        SET up_vote=watch+1
        WHERE post.id = $1
    "#)
        .bind(post_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_up_vote_post.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_up_vote_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    let _ = &trx.commit().await;

    return Ok(saved_up_vote_post.unwrap());
}

pub async fn send_down_vote(
    post_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostDownVote, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        info!(target: "send_down_vote","failed to begin transaction {:?}",open_trx.unwrap_err());
        return Err("gagal membuka transaksi".to_string());
    }
    let mut trx = open_trx.unwrap();


    let is_exist = sqlx::query_as::<_, PostDownVote>(r#"
        SELECT * FROM post_down_vote
        WHERE user_id=$1 AND post_id=$2
    "#)
        .bind(user_id)
        .bind(post_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        info!(target: "send_down_vote","Failed to get data");
        let _ = &trx.rollback().await;
        return Err("Kamu suda like postingan ini".to_string());
    }

    let saved_down_vote_post = sqlx::query_as::<_, PostDownVote>(r#"
        INSERT INTO post_down_vote(user_id,post_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .fetch_one(&mut *trx)
        .await;

    if saved_down_vote_post.is_err() {
        info!(target: "send_like","failed to save dislike reason:{:?}",saved_down_vote_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me dislike postingan.".to_string());
    }

    let increment_down_vote_post = sqlx::query_as::<_, Post>(r#"
        UPDATE post
        SET down_vote=watch+1
        WHERE post.id = $1
    "#)
        .bind(post_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_down_vote_post.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_down_vote_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    return Ok(saved_down_vote_post.unwrap());
}

/// comment
pub async fn get_list_comment(
    page: i32,
    size: i32,
    post_id: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<PostCommentWithUserAndPost> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(r#"
        SELECT CAST(COUNT(id) as INTEGER) FROM post_comment
        WHERE post_comment.post_id = $1 AND post_comment.reply_to_id != null
        ORDER BY created_at DESC
    "#, )
        .bind(post_id)
        .fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_comment","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, PostCommentWithUserAndPost>(r#"
        SELECT *,user,post FROM post_comment
        JOIN user_credential as user ON user.id=post_comment.user_id
        JOIN post as post ON post.id=post_comment.post_id
        WHERE post_comment.post_id = $1 AND post_comment.reply_to_id != null
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    "#)
        .bind(post_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;


    if data.is_err() {
        info!(target:"get_list_comment","{:?}",data.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }
    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    return PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: data.unwrap(),
    };
}

pub async fn send_comment(
    comment: PostComment,
    pool: &Pool<Postgres>,
) -> Result<PostComment, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        info!(target: "send_comment","failed begin transaction");
        return Err("Gagal mengirim comment".to_string());
    }

    let mut trx = open_trx.unwrap();
    let saved_comment = sqlx::query_as::<_, PostComment>(r#"
        INSERT INTO post_comment(post_id,user_id,body)
        VALUES($1,$2,$3)
        RETURNING *
    "#)
        .bind(comment.post_id)
        .bind(comment.user_id)
        .bind(comment.body)
        .fetch_one(&mut *trx)
        .await;
    if saved_comment.is_err() {
        info!(target: "send_comment","{:?}",saved_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("failed to save comment".to_string());
    }

    let increment_comment_post = sqlx::query_as::<_, Post>(r#"
        UPDATE post
        SET comments=comments+1
        WHERE post.id = $1
    "#)
        .bind(comment.post_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_comment_post.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_comment_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    return Ok(saved_comment.unwrap());
}

pub async fn send_reply_comment(
    comment: PostComment,
    pool: &Pool<Postgres>,
) -> Result<PostComment, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        info!(target: "send_reply_comment","failed begin transaction");
        return Err("Gagal begin transaction".to_string());
    }

    let mut trx = open_trx.unwrap();
    let save_reply_comment = sqlx::query_as::<_, PostComment>(r#"
        INSERT INTO post_comment(post_id,user_id,reply_to_id,body)
        VALUES($1,$2,$3,$4)
        RETURNING *
    "#)
        .bind(comment.post_id)
        .bind(comment.user_id)
        .bind(comment.reply_to_id)
        .bind(comment.body)
        .fetch_one(&mut *trx)
        .await;

    if save_reply_comment.is_err() {
        info!(target: "send_comment","{:?}",save_reply_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("failed to save comment".to_string());
    }

    let increment_comment_reply = sqlx::query_as::<_, Post>(r#"
        UPDATE post_comment
        SET reply_count=reply_count+1
        WHERE post_comment.id = $1
    "#)
        .bind(comment.reply_to_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_comment_reply.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_comment_reply.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    let increment_comment_post = sqlx::query_as::<_, Post>(r#"
        UPDATE post
        SET comments=comments+1
        WHERE post.id = $1
    "#)
        .bind(comment.post_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_comment_post.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_comment_post.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    return Ok(save_reply_comment.unwrap());
}


/// comment reply,watch
pub async fn send_watch_comment(
    comment_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostCommentWatch, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        info!(target: "send_watch_comment","failed to begin transaksion {:?}",open_trx.unwrap_err());
        return Err("Gagal megirim watch".to_string());
    }
    let mut trx = open_trx.unwrap();


    let is_exist = sqlx::query_as::<_, PostCommentWatch>(r#"
        SELECT * FROM post_comment_watch
        WHERE user_id=$1 AND post_comment_id=$2
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        info!(target: "send_watch","Failed to get data");
        let _ = &trx.rollback().await;
        return Err("Kamu sudah like postingan ini".to_string());
    }

    let saved_watch_comment = sqlx::query_as::<_, PostCommentWatch>(r#"
        INSERT INTO post_comment_watch(user_id,post_comment_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if saved_watch_comment.is_err() {
        info!(target: "send_like","failed to save like reason:{:?}",saved_watch_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me like postingan.".to_string());
    }

    let increment_watch_comment = sqlx::query_as::<_, Post>(r#"
        UPDATE post_comment
        SET watch=watch+1
        WHERE post_comment.id = $1
    "#)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_watch_comment.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_watch_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    let _ = &trx.commit().await;

    return Ok(saved_watch_comment.unwrap());
}

pub async fn send_up_vote_comment(
    comment_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostCommentUpVote, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        return Err("".to_string());
    }
    let mut trx = open_trx.unwrap();

    let is_exist = sqlx::query_as::<_, PostCommentUpVote>(r#"
        SELECT * FROM post_comment_up_vote
        WHERE user_id=$1 AND post_comment_id=$2
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        return Err("Kamu suda like postingan ini".to_string());
    }

    let save_up_vote_comment = sqlx::query_as::<_, PostCommentUpVote>(r#"
        INSERT INTO post_comment_up_vote(user_id,post_comment_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if save_up_vote_comment.is_err() {
        info!(target: "send_like","failed to save like reason:{:?}",save_up_vote_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me like postingan.".to_string());
    }

    let increment_up_vote_comment = sqlx::query_as::<_, PostComment>(r#"
        UPDATE post_comment
        SET up_vote=watch+1
        WHERE post_comment.id = $1
        RETURNING *
    "#)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_up_vote_comment.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_up_vote_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    let _ = &trx.commit().await;

    return Ok(save_up_vote_comment.unwrap());
}

pub async fn send_down_vote_comment(
    comment_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<PostCommentDownVote, String> {
    let open_trx = pool.begin().await;
    if open_trx.is_err() {
        return Err("".to_string());
    }
    let mut trx = open_trx.unwrap();


    let is_exist = sqlx::query_as::<_, PostCommentDownVote>(r#"
        SELECT * FROM post_comment_down_vote
        WHERE user_id=$1 AND post_comment_id=$2
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_optional(&mut *trx)
        .await.unwrap_or(None);

    if is_exist.is_some() {
        info!(target: "send_down_vote","Failed to get data");
        let _ = &trx.rollback().await;
        return Err("Kamu suda like postingan ini".to_string());
    }

    let saved_down_vote_comment = sqlx::query_as::<_, PostCommentDownVote>(r#"
        INSERT INTO post_comment_down_vote(user_id,post_comment_id)
        VALUES($1,$2)
        RETURNING *
    "#)
        .bind(user_id)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if saved_down_vote_comment.is_err() {
        info!(target: "send_like","failed to save dislike reason:{:?}",saved_down_vote_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal me dislike postingan.".to_string());
    }

    let increment_down_vote_comment = sqlx::query_as::<_, Post>(r#"
        UPDATE post_comment
        SET down_vote=watch+1
        WHERE post_comment.id = $1
    "#)
        .bind(comment_id)
        .fetch_one(&mut *trx)
        .await;

    if increment_down_vote_comment.is_err() {
        info!(target: "send_like","Failed to increment like, rollback {:?}",increment_down_vote_comment.unwrap_err());
        let _ = &trx.rollback().await;
        return Err("Gagal menambah jumlah like".to_string());
    }

    return Ok(saved_down_vote_comment.unwrap());
}