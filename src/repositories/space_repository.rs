use log::info;
use sqlx::{Pool, Postgres};

use crate::common::api_response::{Count, PagingResponse};
use crate::entity::space::{Space, SpaceFollower, SpaceWithUserAndThumbnail};
use crate::entity::user_credential::UserCredentialSecured;

pub async fn get_list_space_with_paging(
    size: i32,
    page: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<SpaceWithUserAndThumbnail> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM space"#,
    ).fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_space","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, SpaceWithUserAndThumbnail>(
        r#"
        SELECT *,user_credential,storage FROM space
        JOIN user_credential ON user_credential.id = space.user_id
        JOIN storage ON storage.id = space.space_thumbnail_id
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
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


    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_space","{:?}",total.clone());
    let users = data.unwrap();
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: users,
    }
}

pub async fn get_list_space_by_current_user(
    size: i32,
    page: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<SpaceWithUserAndThumbnail> {
    let limit = size.clone();
    let offset = limit * (page - 1);

    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM space"#,
    ).fetch_one(pool).await;

    if count.is_err() {
        info!(target:"get_list_space","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages: 0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, SpaceWithUserAndThumbnail>(
        r#"
        SELECT *,user_credential,storage FROM space
        JOIN user_credential ON user_credential.id = space.user_id
        JOIN storage ON storage.id = space.space_thumbnail_id
        WHERE space.user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
        .bind(user_id)
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


    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_space","{:?}",total.clone());
    let users = data.unwrap();
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: users,
    }
}

pub async fn get_space_by_id(
    id:i32,
    pool:&Pool<Postgres>
)->Option<SpaceWithUserAndThumbnail>{
    let find_space = sqlx::query_as::<_, SpaceWithUserAndThumbnail>(r#"
    SELECT * FROM space
    JOIN user_credential ON user_credential = space.user_id
    JOIN storage ON storage.id = post.space_thumbnail_id
    WHERE space.id = $1
    "#)
        .bind(id)
        .fetch_one(pool)
        .await;

    if find_space.is_err() {
        info!(target: "delete_space","{:?}",find_space.unwrap_err());
        return None;
    }
    return Some(find_space.unwrap())

}

pub async fn create_space(
    space: &Space,
    pool: &Pool<Postgres>,
) -> Option<SpaceWithUserAndThumbnail> {
    let saved_space = sqlx::query_as::<_, Space>(r#"
    INSERT INTO space(user_id,name,space_thumbnail_id,is_public,description)
    VALUES($1,$2,$3,$4,$5)
    RETURNING *
    "#)
        .bind(space.user_id)
        .bind(space.name.clone())
        .bind(space.space_thumbnail_id)
        .bind(space.is_public)
        .bind(space.description.clone())
        .fetch_one(pool)
        .await;

    if saved_space.is_err() {
        info!(target: "create_space","{:?}",saved_space.unwrap_err());
        return None;
    }
    let space = saved_space.unwrap();
    let find_space = sqlx::query_as::<_, SpaceWithUserAndThumbnail>(r#"
    SELECT *,user_credential FROM space
    JOIN user_credential ON  user_credential.id = space.user_id
    JOIN storage ON stirage.id = space.space_thumbnail_id
    WHERE space.id = $1
    "#)
        .bind(space.id)
        .fetch_one(pool)
        .await;

    if find_space.is_err() {
        info!(target: "create_space","{:?}",find_space.unwrap_err());
        return None;
    }

    return Some(find_space.unwrap());
}

pub async fn delete_space(
    space_id: i32,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Result<i32, String> {
    let delete = sqlx::query_as::<_, Space>(r#"
        DELETE FROM space
        WHERE space.id=$1
        RETURNING *;
    "#)
        .bind(space_id)
        .fetch_optional(pool)
        .await;

    if delete.is_err() {
        info!(target: "delete_space","{:?}",delete.unwrap_err());
        return Err("Failed to create space".to_string());
    }

    return Ok(space_id);
}

pub async fn follow_space(
    space_follower: SpaceFollower,
    pool: &Pool<Postgres>,
) -> Result<SpaceFollower, String> {
    let saved = sqlx::query_as::<_, SpaceFollower>(r#"
    INSERT INTO space_follower(space_id,user_id)
    VALUES ($1,$2)
    RETURNING *
    "#)
        .bind(space_follower.space_id)
        .bind(space_follower.user_id)
        .fetch_one(pool)
        .await;

    if saved.is_err() {
        info!(target: "delete_space","{:?}",saved.unwrap_err());
        return Err("Failed to create space".to_string());
    }
    let space = saved.unwrap();

    Ok(space)
}