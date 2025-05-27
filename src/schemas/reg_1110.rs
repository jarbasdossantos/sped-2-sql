// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        nr_memo -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
    }
}
