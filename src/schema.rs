// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        image -> Nullable<Varchar>,
        external_link -> Nullable<Varchar>,
        seller_fee_basis_points -> Nullable<Int4>,
        project_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        wallet_id -> Uuid,
    }
}

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

diesel::joinable!(collections -> projects (project_id));
diesel::joinable!(profiles -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(collections, profiles, projects, wallets,);
