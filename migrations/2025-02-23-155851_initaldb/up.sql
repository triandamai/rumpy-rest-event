-- Your SQL goes here
-- Add migration script here
CREATE TABLE IF NOT EXISTS tb_user(
    id SERIAL PRIMARY KEY,
    display_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone_number VARCHAR(255),
    password VARCHAR(255),
    user_meta_data JSON,
    app_meta_data JSON,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
    confirmation_at TIMESTAMP DEFAULT NULL,
    confirmation_sent_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_email ON tb_user(email);

CREATE TABLE IF NOT EXISTS tb_profile_picture(
    user_id INT PRIMARY KEY NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    bucket_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES tb_user(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS tb_topics(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_topic ON tb_topics(name);

CREATE TABLE IF NOT EXISTS tb_thread(
    id SERIAL PRIMARY KEY,
    created_by_id INT NULL,
    quote_thread_id INT NULL,
    reply_to_thread_id INT NULL,
    title TEXT,
    content TEXT,
    up_vote_count INT NULL,
    down_vote_count INT NULL,
    reply_count INT NULL,
    comment_count INT NULL,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY(created_by_id) REFERENCES tb_user(id) ON DELETE SET NULL,
    FOREIGN KEY(reply_to_thread_id) REFERENCES tb_thread(id) ON DELETE SET NULL,
    FOREIGN KEY(quote_thread_id) REFERENCES tb_thread(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS tb_thread_topics(
    thread_id INT NULL,
    topic_id INT NULL,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (thread_id, topic_id),
    FOREIGN KEY (thread_id) REFERENCES tb_thread(id) ON DELETE SET NULL,
    FOREIGN KEY (topic_id) REFERENCES tb_topics(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS tb_thread_polling(
    id SERIAL PRIMARY KEY,
    thread_id INT NULL,
    slug VARCHAR(255),
    vote_count BIGINT,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS tb_thread_attachment(
    id SERIAL PRIMARY KEY,
    thread_id INT NULL,
    kind VARCHAR(255) NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    bucket_name VARCHAR(255) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY(thread_id) REFERENCES tb_thread(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS tb_audit_log(
    id SERIAL PRIMARY KEY,
    event_id VARCHAR(255) NOT NULL,
    content JSON,
    created_at TIMESTAMP DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);
