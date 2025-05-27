// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
    }
}
