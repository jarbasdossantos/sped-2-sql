// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
    }
}
