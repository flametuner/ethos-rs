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
