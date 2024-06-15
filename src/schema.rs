// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (id) {
        id -> Int4,
        network_id -> Varchar,
        block_number -> Int8,
        timestamp -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
