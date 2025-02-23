// @generated automatically by Diesel CLI.

diesel::table! {
    tb_audit_log (id) {
        id -> Int4,
        #[max_length = 255]
        event_id -> Varchar,
        content -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_profile_picture (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        mime_type -> Varchar,
        #[max_length = 255]
        file_name -> Varchar,
        #[max_length = 255]
        bucket_name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_thread (id) {
        id -> Int4,
        created_by_id -> Nullable<Int4>,
        quote_thread_id -> Nullable<Int4>,
        reply_to_thread_id -> Nullable<Int4>,
        title -> Nullable<Text>,
        content -> Nullable<Text>,
        up_vote_count -> Nullable<Int4>,
        down_vote_count -> Nullable<Int4>,
        reply_count -> Nullable<Int4>,
        comment_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_thread_attachment (id) {
        id -> Int4,
        thread_id -> Nullable<Int4>,
        #[max_length = 255]
        kind -> Varchar,
        #[max_length = 255]
        mime_type -> Varchar,
        #[max_length = 255]
        bucket_name -> Varchar,
        #[max_length = 255]
        file_name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_thread_polling (id) {
        id -> Int4,
        thread_id -> Nullable<Int4>,
        #[max_length = 255]
        slug -> Nullable<Varchar>,
        vote_count -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_thread_topics (thread_id, topic_id) {
        thread_id -> Int4,
        topic_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_topics (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tb_user (id) {
        id -> Int4,
        #[max_length = 255]
        display_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        user_meta_data -> Nullable<Json>,
        app_meta_data -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        confirmation_at -> Nullable<Timestamp>,
        confirmation_sent_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(tb_profile_picture -> tb_user (user_id));
diesel::joinable!(tb_thread -> tb_user (created_by_id));
diesel::joinable!(tb_thread_attachment -> tb_thread (thread_id));
diesel::joinable!(tb_thread_topics -> tb_thread (thread_id));
diesel::joinable!(tb_thread_topics -> tb_topics (topic_id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_audit_log,
    tb_profile_picture,
    tb_thread,
    tb_thread_attachment,
    tb_thread_polling,
    tb_thread_topics,
    tb_topics,
    tb_user,
);
