-- Add migration script here

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'conversation_type') THEN
        CREATE TYPE conversation_type AS ENUM ('group', 'direct', 'other');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_status') THEN
        CREATE TYPE user_status AS ENUM ('active', 'inactive', 'waitingconfirmation', 'suspended', 'lock');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'auth_provider') THEN
        CREATE TYPE auth_provider AS ENUM ('basic', 'google', 'facebook', 'apple', 'twitter');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'message_type') THEN
        CREATE TYPE message_type AS ENUM ('text', 'sticker', 'image','other');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'attachment_type') THEN
        CREATE TYPE attachment_type AS ENUM ('image', 'video', 'file', 'other');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status_message') THEN
        CREATE TYPE status_message AS ENUM('sent', 'delivered', 'read');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'post_type') THEN
            CREATE TYPE post_type AS ENUM('polling', 'thought', 'none');
    END IF;
END $$;
-- UserCredential
CREATE TABLE IF NOT EXISTS user_credential (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    status user_status NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    username VARCHAR(255) NOT NULL UNIQUE,
    uuid TEXT NOT NULL,
    deleted BOOLEAN DEFAULT FALSE,
    auth_provider auth_provider NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_user_credential_status ON user_credential(status);
CREATE INDEX IF NOT EXISTS idx_user_credential_email ON user_credential(email);
CREATE INDEX IF NOT EXISTS idx_user_credential_username ON user_credential(username);

-- Conversation
CREATE TABLE IF NOT EXISTS conversation (
    id SERIAL PRIMARY KEY,
    conversation_name VARCHAR(255) NOT NULL,
    conversation_type conversation_type NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

--Conversation Member
CREATE TABLE IF NOT EXISTS conversation_member (
    user_id INT NOT NULL,
    conversation_id INT NOT NULL,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, conversation_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversation(id) ON DELETE SET NULL
);

--Message
CREATE TABLE IF NOT EXISTS message (
    id SERIAL PRIMARY KEY,
    sender_id INT NOT NULL,
    conversation_id INT NOT NULL,
    message_content TEXT NOT NULL,
    message_type message_type NOT NULL,
    message_sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (conversation_id) REFERENCES conversation(id) ON DELETE SET NULL,
    FOREIGN KEY (sender_id) REFERENCES user_credential(id) ON DELETE SET NULL
);

--Message Status
CREATE TABLE IF NOT EXISTS message_status (
    message_id INT NOT NULL,
    user_id INT NOT NULL,
    conversation_id INT NOT NULL,
    message_status status_message NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (message_id, user_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (message_id) REFERENCES message(id) ON DELETE SET NULL
);

--Message Attachment
CREATE TABLE IF NOT EXISTS message_attachment (
    id SERIAL PRIMARY KEY,
    message_id INT NOT NULL,
    attachment_type attachment_type NOT NULL,
    attachment_url TEXT NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (message_id) REFERENCES message(id) ON DELETE SET NULL
);


--Friend
CREATE TABLE IF NOT EXISTS friend (
    friend_id INT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (friend_id, user_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (friend_id) REFERENCES user_credential(id) ON DELETE SET NULL
);


CREATE TABLE IF NOT EXISTS friend_request (
    id SERIAL PRIMARY KEY,
    from_id INT NOT NULL,
    friend_id INT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_rejected BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (from_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (friend_id) REFERENCES user_credential(id) ON DELETE SET NULL
);

-- STORAGE
CREATE TABLE IF NOT EXISTS storage(
    id SERIAL PRIMARY KEY,
    file_name VARCHAR(255),
    bucket VARCHAR(255),
    mime_type VARCHAR(255),
    is_used BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- SPACE
CREATE TABLE IF NOT EXISTS space(
    id SERIAL PRIMARY KEY,
    user_id INT NULL,
    name VARCHAR(255) NOT NULL,
    space_thumbnail_id INT NULL,
    is_public BOOLEAN NOT NULL DEFAULT true,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (space_thumbnail_id) REFERENCES storage(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS space_follower(
    space_id INT NULL,
    user_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (space_id,user_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (space_id) REFERENCES space(id) ON DELETE SET NULL
);

-- POST
CREATE TABLE IF NOT EXISTS post(
    id SERIAL PRIMARY KEY,
    user_id INT NULL,
    space_id INT NULL,
    body TEXT NOT NULL,
    post_type post_type NOT NULL,
    watch BIGINT NOT NULL DEFAULT 0,
    comments BIGINT NOT NULL DEFAULT 0,
    up_vote BIGINT NOT NULL DEFAULT 0,
    down_vote BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (space_id) REFERENCES space(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_watch(
    user_id INT NOT NULL,
    post_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_up_vote(
    user_id INT NOT NULL,
    post_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_down_vote(
    user_id INT NOT NULL,
    post_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_comment(
    id SERIAL PRIMARY KEY,
    post_id INT,
    user_id INT,
    reply_to_id INT NULL,
    body TEXT NOT NULL,
    reply_count BIGINT NOT NULL DEFAULT 0,
    watch BIGINT NOT NULL DEFAULT 0,
    up_vote BIGINT NOT NULL DEFAULT 0,
    down_vote BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE SET NULL,
    FOREIGN KEY (reply_to_id) REFERENCES post_comment(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_comment_watch(
    user_id INT NOT NULL,
    post_comment_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_comment_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_comment_id) REFERENCES post_comment(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_comment_up_vote(
    user_id INT NOT NULL,
    post_comment_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_comment_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_comment_id) REFERENCES post_comment(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_comment_down_vote(
    user_id INT NOT NULL,
    post_comment_id INT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id,post_comment_id),
    FOREIGN KEY (user_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (post_comment_id) REFERENCES post_comment(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS post_attachment(
    post_id INT,
    file_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (post_id,file_id),
    FOREIGN KEY (post_id) REFERENCES user_credential(id) ON DELETE SET NULL,
    FOREIGN KEY (file_id) REFERENCES storage(id) ON DELETE SET NULL
);



