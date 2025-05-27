// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        pis_imp -> Nullable<Text>,
        cofins_imp -> Nullable<Text>,
        num_acdraw -> Nullable<Text>,
    }
}
