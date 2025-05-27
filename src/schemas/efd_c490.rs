// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c490 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
    }
}
