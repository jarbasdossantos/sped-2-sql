// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0450 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}
