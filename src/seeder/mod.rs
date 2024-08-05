use bcrypt::DEFAULT_COST;
use sqlx::{Pool, Postgres};
use crate::entity::conversation::{Conversation, ConversationMember, ConversationType};
use crate::entity::user_credential::{AuthProvider, UserCredential, UserStatus};

pub async fn seed(
    pool: &Pool<Postgres>
) {
    let uuid = uuid::Uuid::new_v4();
    let password = bcrypt::hash("12345678", DEFAULT_COST).unwrap();

    let user = UserCredential {
        id: 1,
        uuid: uuid.to_string(),
        username: "trian".to_string(),
        password: password.clone(),
        full_name: "n/a".to_string(),
        email: "trian1@email.com".to_string(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
        status: UserStatus::Active,
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let user2 = UserCredential {
        id: 2,
        uuid: uuid.to_string(),
        username: "parzival".to_string(),
        password: password.clone(),
        full_name: "n/a".to_string(),
        email: "parzival@email.com".to_string(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
        status: UserStatus::WaitingConfirmation,
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let query = r#"
            INSERT INTO user_credential(
               id,uuid,username,password,full_name,email,deleted,auth_provider,status,created_at,updated_at
            ) VALUES ($1,$2, $3, $4,$5,$6,$7,$8,$9,$10,$11)
            RETURNING *
            "#;
    let query_select_user = "SELECT * FROM user_credential WHERE id=$1";

    let find = sqlx::query_as::<_, UserCredential>(query_select_user)
        .bind(user.id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None);
    if find.is_none() {
        let _ = sqlx::query_as::<_, UserCredential>(query)
            .bind(user.id)
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
    }

    let find2 = sqlx::query_as::<_, UserCredential>(query_select_user)
        .bind(user2.id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None);
    if find2.is_none() {
        let _ = sqlx::query_as::<_, UserCredential>(query)
            .bind(user2.id)
            .bind(user2.uuid)
            .bind(user2.username)
            .bind(user2.password)
            .bind(user2.full_name)
            .bind(user2.email)
            .bind(user2.deleted)
            .bind(user2.auth_provider)
            .bind(user2.status)
            .bind(user2.created_at)
            .bind(user2.updated_at)
            .fetch_one(pool)
            .await;
    }

    let conversation = Conversation {
        id: 1,
        conversation_name: "".to_string(),
        conversation_type: ConversationType::Direct,
        created_at: chrono::Utc::now().naive_local(),
    };

    let find = sqlx::query_as::<_, Conversation>(
        "SELECT * FROM conversation WHERE id=$"
    )
        .bind(conversation.id)
        .fetch_optional(pool)
        .await.unwrap_or(None);
    if find.is_none() {
        let _ = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversation(
                id,conversation_name,conversation_type,created_at
            ) VALUES ($1,$2,$3,$4) RETURNING *
            "#
        )
            .bind(conversation.id)
            .bind(conversation.conversation_name)
            .bind(conversation.conversation_type)
            .bind(conversation.created_at)
            .fetch_optional(pool)
            .await;

        let txn = pool.begin().await;
        if txn.is_ok() {
            let mut tx = txn.unwrap();
            let members = vec![
                ConversationMember {
                    user_id: user.id,
                    joined_at: chrono::Utc::now().naive_local(),
                    conversation_id: conversation.id,
                },
                ConversationMember {
                    user_id: user2.id,
                    joined_at: chrono::Utc::now().naive_local(),
                    conversation_id: conversation.id,
                },
            ];

            for member in &members {
                let _ = sqlx::query(
                    "INSERT INTO conversation_member (user_id, conversation_id, joined_at) VALUES ($1, $2, $3)"
                )
                    .bind(member.user_id)
                    .bind(member.conversation_id)
                    .bind(member.joined_at)
                    .execute(&mut *tx)
                    .await;
            }
            let _ = tx.commit().await;
        }
    }
}