-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users" (
  "id" UUID PRIMARY KEY,
  "username" VARCHAR(255) UNIQUE NOT NULL,
  "password_hash" TEXT NOT NULL
);
