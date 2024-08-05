use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use itertools::Itertools;
use log::{info, trace};
use sqlx::{Pool, Postgres};

use crate::entity::conversation::{Conversation, ConversationMember, ConversationMemberWithUser, ConversationMemberWithUserAndConversation, ConversationType, ConversationWithMember};
use crate::entity::message::{Message, MessageWithAttachment};

pub async fn get_conversations_by_user_id(
    user_id: i32,
    pool: &Pool<Postgres>,
) -> Vec<ConversationWithMember> {
    let query = r#"
    SELECT member.*,conversation,account
    FROM conversation
    JOIN conversation_member member ON conversation.id = member.conversation_id
    JOIN user_credential as account ON member.user_id = account.id
    WHERE
         CASE
	        WHEN conversation.conversation_type = 'direct'
	            THEN
		            conversation.id IN (SELECT conversation_id FROM conversation_member WHERE user_id = $1)
         ELSE
                member.user_id = $1
         END
    ORDER BY conversation.created_at, member.joined_at;
    "#;

    let data =
        sqlx::query_as::<_, ConversationMemberWithUserAndConversation>(query)
            .bind(user_id).fetch_all(pool).await;

    if data.is_err() {
        info!(target: "conversation_repo::get_conversations_by_user_id","{}",data.unwrap_err().to_string());
        return Vec::new();
    }
    let conversations = data.unwrap();
    let mut lookup = HashMap::<i32, Vec<ConversationMemberWithUserAndConversation>>::new();
    let mut conversation_group = Vec::<ConversationWithMember>::new();

    for member in conversations {
        lookup.entry(member.conversation_id).or_insert_with(Vec::new).push(member);
    }

    for (key, element) in lookup {
        let take = element.first();
        match take {
            None => {}
            Some(data) => {
                let result: Vec<ConversationMemberWithUser> = element.clone().into_iter()
                    .map(|value| {
                        return ConversationMemberWithUser {
                            user_id: value.user_id,
                            joined_at: value.joined_at,
                            conversation_id: key,
                            account: value.account,
                        };
                    }).collect();
                let conversation = data.clone();
                conversation_group.push(ConversationWithMember {
                    id: key,
                    conversation_name: conversation.conversation.conversation_name,
                    conversation_type: conversation.conversation.conversation_type,
                    created_at: conversation.conversation.created_at,
                    members: result,
                })
            }
        }
    }
    conversation_group
}

pub async fn get_messages_by_conversation_id(
    conversation_id: i32,
    from: NaiveDateTime,
    to: NaiveDateTime,
    pool: &Pool<Postgres>,
) -> Vec<MessageWithAttachment> {
    let data = sqlx::query_as::<_, MessageWithAttachment>(
        r#"
        SELECT DISTINCT message.*,attachment FROM message
        FULL JOIN message_attachment as attachment
        ON attachment.message_id = message.id
        WHERE message.message_sent_at BETWEEN $1 AND $2
        AND message.conversation_id=$3
        ORDER BY message.message_sent_at DESC
        "#
    )
        .bind(from)
        .bind(to)
        .bind(conversation_id)
        .fetch_all(pool)
        .await;

    if data.is_err() {
        info!(target: "conversation_repo::get_messages_by_conversation_id","{}",data.unwrap_err().to_string());
        return Vec::new();
    }
    data.unwrap()
}

pub async fn search_direct_conversation_if_exists(
    members: Vec<i32>,
    pool: &Pool<Postgres>,
) -> Vec<ConversationMemberWithUserAndConversation> {
    let data = sqlx::query_as::<_, ConversationMemberWithUserAndConversation>(
        r#"
        SELECT member.*,account,conversation FROM conversation_member as member
        LEFT JOIN conversation as conversation ON conversation.id = member.conversation_id
        LEFT JOIN user_credential as account ON account.id = member.user_id
        WHERE member.user_id IN (SELECT unnest($1::integer[])) AND conversation.conversation_type = 'direct'
        GROUP BY member.user_id,member.conversation_id,account.id,conversation.id
         "#
    )
        .bind(members)
        .fetch_all(pool)
        .await;
    if data.is_err() {
        info!(target: "conversation_repo::search_direct_conversation_if_exists","{}",data.unwrap_err().to_string());
        return Vec::new();
    }
    data.unwrap()
}

pub async fn get_conversation_with_member(
    conversation_id: i32,
    pool: &Pool<Postgres>,
) -> Option<ConversationWithMember> {
    let conversation = sqlx::query_as::<_, Conversation>(
        "SELECT * FROM conversation WHERE id=$1"
    )
        .bind(conversation_id)
        .fetch_optional(pool)
        .await;

    if conversation.is_err() {
        info!(target: "conversation_repo","{}" ,conversation.unwrap_err().to_string());
        return None;
    }

    let conversation = conversation.unwrap();
    if conversation.is_none() {
        return None;
    }
    let conversation = conversation.unwrap();

    let conversation_members = sqlx::query_as::<_, ConversationMemberWithUser>(
        r#"
            SELECT *,account FROM conversation_member
            INNER JOIN user_credential as account ON account.id = conversation_member.user_id
            WHERE conversation_member.conversation_id=$1
        "#
    )
        .bind(conversation_id)
        .fetch_all(pool)
        .await;

    if conversation_members.is_err() {
        info!(target: "conversation_repo","{}" ,conversation_members.unwrap_err().to_string());
        return None;
    }

    let conversation_members = conversation_members.unwrap_or(Vec::new());
    Some(ConversationWithMember {
        id: conversation.id,
        conversation_name: conversation.conversation_name,
        conversation_type: conversation.conversation_type,
        created_at: conversation.created_at,
        members: conversation_members,
    })
}

pub async fn create_conversation(
    conversation_type: ConversationType,
    members: Vec<i32>,
    pool: &Pool<Postgres>,
) -> Option<Conversation> {
    let txn = pool.begin().await;
    if txn.is_err() {
        return None;
    }

    let current_date = chrono::Utc::now().naive_local();
    let mut tx = txn.unwrap();

    let saved_conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversation (conversation_name, conversation_type,created_at)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
        .bind("n/a".to_string())
        .bind(conversation_type)
        .bind(current_date)
        .fetch_one(&mut *tx)
        .await;


    if saved_conversation.is_err() {
        info!(target: "conversation_repo","{}" ,saved_conversation.unwrap_err().to_string());
        let _ = &tx.rollback().await;
        return None;
    }

    let conversation = saved_conversation.unwrap();

    let members: Vec<ConversationMember> = members.iter().map(|req| {
        ConversationMember {
            conversation_id: conversation.id,
            user_id: req.clone(),
            joined_at: current_date,
        }
    }).collect();

    let saved_member_query = r#"
        INSERT INTO conversation_member(conversation_id, user_id, joined_at)
        SELECT * FROM unnest($1::int[], $2::int[], $3::timestamp[])
    "#;

    let conversation_ids: Vec<i32> = members.iter().map(|v| v.conversation_id).collect();
    let user_ids: Vec<i32> = members.iter().map(|v| v.user_id).collect();
    let joined_ats: Vec<NaiveDateTime> = members.iter().map(|v| v.joined_at).collect();

    let saved_member = sqlx::query(saved_member_query)
        .bind(conversation_ids)
        .bind(user_ids)
        .bind(joined_ats)
        .execute(&mut *tx)
        .await;

    if saved_member.is_err() {
        info!(target: "conversation_repo","{}" ,saved_member.unwrap_err().to_string());
        let _ = tx.rollback().await;
        return None;
    }

    let start = tx.commit().await;

    if start.is_err() {
        info!(target: "conversation_repo","{}" ,start.unwrap_err().to_string());
        return None;
    }
    Some(conversation)
}


pub async fn create_message(
    message: Message,
    pool: &Pool<Postgres>,
) -> Option<Message> {
    let saved_message = sqlx::query_as::<_, Message>(
        r#"INSERT INTO message(
            conversation_id,
            sender_id,
            message_content,
            message_type,
            message_sent_at,
            updated_at
        ) VALUES($1,$2,$3,$4,$5,$6) RETURNING *"#
    )
        .bind(message.conversation_id)
        .bind(message.sender_id)
        .bind(message.message_content)
        .bind(message.message_type)
        .bind(message.message_sent_at)
        .bind(message.updated_at)
        .fetch_one(pool)
        .await;


    if saved_message.is_err() {
        info!(target: "conversation_repo","{}" ,saved_message.unwrap_err().to_string());
        return None;
    }

    return Some(saved_message.unwrap());
}