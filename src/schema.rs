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
