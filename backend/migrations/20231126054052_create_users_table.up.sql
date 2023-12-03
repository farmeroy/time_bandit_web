-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  uuid UUID DEFAULT gen_random_uuid() UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE
);
