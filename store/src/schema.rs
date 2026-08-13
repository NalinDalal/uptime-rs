//! Diesel schema definitions.
//!
//! Auto-generated table definitions mapping Rust structs to PostgreSQL tables.
//! Includes joins and allowlists for cross-table queries.

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        password -> Text,
        webhook_url -> Nullable<Text>,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamp,
        user_id -> Text,
        component -> Nullable<Text>,
    }
}

diesel::table! {
    website_tick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> Text,
        http_status -> Nullable<Int4>,
        region_id -> Text,
        website_id -> Text,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    incident (id) {
        id -> Text,
        website_id -> Text,
        region_id -> Text,
        started_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    maintenance (id) {
        id -> Text,
        website_id -> Text,
        title -> Text,
        description -> Text,
        starts_at -> Timestamp,
        ends_at -> Nullable<Timestamp>,
        status -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(website -> user (user_id));
diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));
diesel::joinable!(incident -> website (website_id));
diesel::joinable!(incident -> region (region_id));
diesel::joinable!(maintenance -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    region,
    user,
    website,
    website_tick,
    incident,
    maintenance,
);
