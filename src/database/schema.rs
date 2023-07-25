// @generated automatically by Diesel CLI.

diesel::table! {
    cronjobs (id) {
        id -> Uuid,
        schedule -> Varchar,
        service -> Varchar,
        status -> Varchar,
        route -> Varchar,
        since -> Nullable<Timestamptz>,
        until -> Nullable<Timestamptz>,
    }
}
