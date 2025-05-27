// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c199 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        vl_pis_imp -> Nullable<Text>,
        vl_cofins_imp -> Nullable<Text>,
        num_acdraw -> Nullable<Text>,
    }
}
