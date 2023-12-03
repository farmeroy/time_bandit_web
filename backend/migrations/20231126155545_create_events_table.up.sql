-- Add up migration script here
CREATE TABLE IF NOT EXISTS events (
  id SERIAL PRIMARY KEY,
  uuid uuid DEFAULT gen_random_uuid() UNIQUE,
  user_id SERIAL NOT NULL,
  task_id SERIAL NOT NULL,
  notes TEXT,
  date_began TIMESTAMPTZ NOT NULL,
  duration BIGINT NOT NULL
);
