// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k230 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini_op -> Nullable<Text>,
        dt_fin_op -> Nullable<Text>,
        cod_doc_op -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd_enc -> Nullable<Text>,
    }
}
