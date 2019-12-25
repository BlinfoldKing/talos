-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "posts" (
    "id" UUID PRIMARY KEY,
    "slug" VARCHAR(30) UNIQUE NOT NULL,
    "title" VARCHAR(30) NOT NULL,
    "content" TEXT NOT NULL,
    "banner" VARCHAR(255) NOT NULL,
    "is_draft" BOOLEAN NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "deleted_at" TIMESTAMP
);
