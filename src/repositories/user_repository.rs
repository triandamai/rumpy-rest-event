use log::info;
use sqlx::{Pool, Postgres};

use crate::{
    common::api_response::{Count, PagingResponse},
    entity::user_credential::UserCredential,
};
use crate::entity::user_credential::UserCredentialSecured;

pub async fn get_list_user(
    page: i32,
    size: i32,
    pool: &Pool<Postgres>,
) -> PagingResponse<UserCredentialSecured> {
    let limit = size.clone();
    let offset = limit * (page - 1);


    let count = sqlx::query_as::<_, Count>(
        r#"SELECT CAST(COUNT(id) as INTEGER) as count FROM user_credential"#,
    )
        .fetch_one(pool)
        .await;
    if count.is_err() {
        info!(target:"get_list_user","{:?}",count.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages:0,
            items: Vec::new(),
        };
    }

    let data = sqlx::query_as::<_, UserCredentialSecured>(
        r#"
        SELECT * FROM user_credential
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target:"get_list_user","{:?}",data.unwrap_err());
        return PagingResponse {
            total_items: 0,
            total_pages:0,
            items: Vec::new(),
        };
    }


    let total = count.unwrap();
    let total_page = (total.clone().count / limit);

    info!(target:"get_list_user","{:?}",total.clone());
    let users = data.unwrap();
    PagingResponse {
        total_items: total.count,
        total_pages: total_page,
        items: users,
    }
}

pub async fn get_user_by_username(
    username: &String,
    pool: &Pool<Postgres>,
) -> Vec<UserCredentialSecured> {
    if username.len() < 1 {
        info!(target: "get_user_by_username","username is blank");
        return Vec::new();
    }
    let data = sqlx::query_as::<_, UserCredentialSecured>(r#"
        SELECT * FROM user_credential
        WHERE username LIKE $1
    "#)
        .bind(format!("%{}%",username))
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target: "get_user_by_username","{:?}",data.unwrap_err());
        return Vec::new();
    }


    return data.unwrap();
}