// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c860 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        doc_ini -> Nullable<Text>,
        doc_fim -> Nullable<Text>,
    }
}
