-- Your SQL goes here
CREATE TABLE gossips(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    kind VARCHAR DEFAULT 'CONVERSATION', -- CONVERSATION, CHANNEL, GROUP, GUILD
    target_id INTEGER DEFAULT NULL, -- it can be user_id, channel_id, group_id or guild_id that depends on type
    last_message_id INTEGER NOT NULL,
    unread_messages INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT null
)