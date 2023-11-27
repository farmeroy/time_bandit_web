-- Add up migration script here
CREATE TABLE IF NOT EXISTS tasks (
  id SERIAL PRIMARY KEY,
  uuid uuid DEFAULT gen_random_uuid() UNIQUE,
  user_id SERIAL NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  created_on TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
