// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c380 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_doc_canc -> Nullable<Text>,
    }
}
