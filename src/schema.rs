table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        to_user_id -> Int4,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        status -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    profiles,
);
