-- Add up migration script here
ALTER TABLE users
ADD COLUMN password VARCHAR;
