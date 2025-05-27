// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        descr_nat -> Nullable<Text>,
    }
}
