// @generated automatically by Diesel CLI.

diesel::table! {
    efd_p990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_p -> Nullable<Text>,
    }
}
