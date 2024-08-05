use log::info;
use sqlx::{Pool, Postgres};

use crate::entity::user_credential::{UserCredential, UserCredentialSecured};

pub async fn get_user_by_id(user_id: i32, pool: &Pool<Postgres>) -> Option<UserCredentialSecured> {
    sqlx::query_as::<_, UserCredentialSecured>("SELECT * FROM user_credential WHERE id=$1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
}

pub async fn get_user_by_id_unsecured(user_id: i32, pool: &Pool<Postgres>) -> Option<UserCredential> {
    sqlx::query_as::<_, UserCredential>("SELECT * FROM user_credential WHERE id=$1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
}

pub async fn get_user_by_email(email: String, pool: &Pool<Postgres>) -> Option<UserCredentialSecured> {
    sqlx::query_as::<_, UserCredentialSecured>("SELECT * FROM user_credential WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
}

pub async fn get_user_by_email_unsecured(email: String, pool: &Pool<Postgres>) -> Option<UserCredential> {
    sqlx::query_as::<_, UserCredential>("SELECT * FROM user_credential WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
}

pub async fn create_new_user(
    user: UserCredential,
    pool: &Pool<Postgres>,
) -> Result<UserCredentialSecured, String> {
    let saved_user = sqlx::query_as::<_, UserCredentialSecured>(
        r#"
        INSERT INTO user_credential(
        uuid,
        username,
        password,
        full_name,
        email,
        deleted,
        auth_provider,
        status,
        created_at,
        updated_at
        ) VALUES (
            $1,$2, $3, $4,$5,$6,$7,$8,$9,$10
        )
        RETURNING *
        "#,
    )
    .bind(user.uuid)
    .bind(user.username)
    .bind(user.password)
    .bind(user.full_name)
    .bind(user.email)
    .bind(user.deleted)
    .bind(user.auth_provider)
    .bind(user.status)
    .bind(user.created_at)
    .bind(user.updated_at)
    .fetch_one(pool)
    .await;

    if saved_user.is_err() {
        info!(target: "create_new_user","{}" ,saved_user.unwrap_err().to_string());
        return Err("Failed to create user".to_string());
    }
    return Ok(saved_user.unwrap());
}
