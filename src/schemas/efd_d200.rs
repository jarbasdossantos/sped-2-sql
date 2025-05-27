// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cfop -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
    }
}
