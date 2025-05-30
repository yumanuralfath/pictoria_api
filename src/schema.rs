// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Int4,
        sender_id -> Int4,
        receiver_id -> Int4,
        message -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

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

diesel::table! {
    voices (id) {
        id -> Int4,
        user_id -> Int4,
        voices_journal -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    voices_months (id) {
        id -> Int4,
        user_id -> Int4,
        voice_id -> Int4,
        voices_month_journal -> Text,
        #[max_length = 7]
        month -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    voices_weeks (id) {
        id -> Int4,
        user_id -> Int4,
        voices_week_journal -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    voices_weeks_voices (id) {
        id -> Int4,
        voices_week_id -> Int4,
        voice_id -> Int4,
    }
}

diesel::joinable!(comments -> threads (thread_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(threads -> users (user_id));
diesel::joinable!(voices -> users (user_id));
diesel::joinable!(voices_months -> users (user_id));
diesel::joinable!(voices_months -> voices_weeks (voice_id));
diesel::joinable!(voices_weeks -> users (user_id));
diesel::joinable!(voices_weeks_voices -> voices (voice_id));
diesel::joinable!(voices_weeks_voices -> voices_weeks (voices_week_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    comments,
    threads,
    users,
    voices,
    voices_months,
    voices_weeks,
    voices_weeks_voices,
);
