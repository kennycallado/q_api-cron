// @generated automatically by Diesel CLI.

diesel::table! {
    cronjobs (id) {
        id -> Int4,
        owner -> Varchar,
        service -> Varchar,
        route -> Varchar,
        job_id -> Uuid,
    }
}

diesel::table! {
    escalonjobs (id) {
        id -> Uuid,
        status -> Varchar,
        schedule -> Varchar,
        since -> Nullable<Timestamptz>,
        until -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(cronjobs -> escalonjobs (job_id));

diesel::allow_tables_to_appear_in_same_query!(cronjobs, escalonjobs,);
