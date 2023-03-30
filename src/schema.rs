// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "display_type"))]
    pub struct DisplayType;
}

diesel::table! {
    attributes_on_nfts (nft_id, attribute_id) {
        nft_id -> Uuid,
        attribute_id -> Uuid,
    }
}

diesel::table! {
    collection_contracts (id) {
        id -> Uuid,
        contract_id -> Nullable<Uuid>,
        address -> Varchar,
        fee_recipient -> Varchar,
        collection_id -> Uuid,
        network_id -> Uuid,
    }
}

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
    networks (id) {
        id -> Uuid,
        chain_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DisplayType;

    nft_attributes (id) {
        id -> Uuid,
        trait_type -> Nullable<Varchar>,
        value -> Nullable<Varchar>,
        max_value -> Nullable<Varchar>,
        display_type -> Nullable<DisplayType>,
    }
}

diesel::table! {
    nfts (id) {
        id -> Uuid,
        nft_id -> Int4,
        name -> Varchar,
        description -> Text,
        minted_at -> Nullable<Timestamp>,
        image -> Varchar,
        external_url -> Varchar,
        animation_url -> Varchar,
        owner_id -> Nullable<Uuid>,
        collection_id -> Uuid,
        network_contract_id -> Uuid,
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

diesel::joinable!(attributes_on_nfts -> nft_attributes (attribute_id));
diesel::joinable!(attributes_on_nfts -> nfts (nft_id));
diesel::joinable!(collection_contracts -> collections (collection_id));
diesel::joinable!(collection_contracts -> networks (network_id));
diesel::joinable!(collections -> projects (project_id));
diesel::joinable!(nfts -> collection_contracts (network_contract_id));
diesel::joinable!(nfts -> collections (collection_id));
diesel::joinable!(nfts -> wallets (owner_id));
diesel::joinable!(profiles -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    attributes_on_nfts,
    collection_contracts,
    collections,
    networks,
    nft_attributes,
    nfts,
    profiles,
    projects,
    wallets,
);
