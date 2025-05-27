// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0206 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comb -> Nullable<Text>,
    }
}
