-- Your SQL goes here
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  sender INTEGER NOT NULL,
  receiver INTEGER NOT NULL,
  content VARCHAR NOT NULL,
  delivered BOOLEAN NOT NULL DEFAULT FALSE,
  deleted_delivered BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
)