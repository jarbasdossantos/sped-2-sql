// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
