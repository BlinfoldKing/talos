table! {
    posts (id) {
        id -> Uuid,
        slug -> Varchar,
        title -> Varchar,
        content -> Text,
        banner -> Varchar,
        author_id -> Uuid,
        is_draft -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
