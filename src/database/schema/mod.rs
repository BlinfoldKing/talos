table! {
    posts (id) {
        id -> Uuid,
        slug -> Varchar,
        title -> Varchar,
        content -> Text,
        banner -> Varchar,
        is_draft -> Bool,
        prev_id -> Nullable<Uuid>,
        next_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
