-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "posts" (
    "id" UUID PRIMARY KEY,
    "slug" VARCHAR(121) UNIQUE NOT NULL,
    "title" VARCHAR(100) NOT NULL,
    "content" TEXT NOT NULL,
    "banner" VARCHAR(255) NOT NULL,
    "is_draft" BOOLEAN NOT NULL,
    "prev_id" UUID,
    "next_id" UUID,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "deleted_at" TIMESTAMP
);

CREATE INDEX "posts_index" ON "posts" (slug);
