// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        ind_escri -> Nullable<Text>,
    }
}
