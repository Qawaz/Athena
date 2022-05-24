-- Your SQL goes here
CREATE TABLE jwt_tokens(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    access_token VARCHAR UNIQUE NOT NULL,
    access_token_expires_at TIMESTAMP NOT NULL,
    refresh_token VARCHAR UNIQUE NOT NULL,
    refresh_token_expires_at TIMESTAMP NOT NULL
)