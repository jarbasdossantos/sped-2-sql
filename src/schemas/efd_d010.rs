// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}
