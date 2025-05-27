// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e113 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Text>,
    }
}
