// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c180 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        vl_tot_item -> Nullable<Text>,
    }
}
