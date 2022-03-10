-- Your SQL goes here
CREATE TABLE followers(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    following INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)