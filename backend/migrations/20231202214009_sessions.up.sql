-- Add up migration script here

CREATE TABLE IF NOT EXISTS sessions (
  id SERIAL PRIMARY KEY,
  session_id VARCHAR NOT NULL UNIQUE,
  user_id INT NOT NULL UNIQUE
);
