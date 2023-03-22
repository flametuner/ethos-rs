// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        url -> Nullable<Varchar>,
        cors -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wallets (id) {
        id -> Uuid,
        address -> Varchar,
        nonce -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    wallets,
);
