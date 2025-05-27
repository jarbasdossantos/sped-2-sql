// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        ccus -> Nullable<Text>,
    }
}
