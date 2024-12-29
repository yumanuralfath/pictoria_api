// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        user_id -> Int4,
        thread_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    threads (id) {
        id -> Int4,
        content -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        profile_picture_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(comments -> threads (thread_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(threads -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    threads,
    users,
);
