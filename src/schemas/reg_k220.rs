// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_mov -> Nullable<Text>,
        cod_item_ori -> Nullable<Text>,
        cod_item_dest -> Nullable<Text>,
        qtd -> Nullable<Text>,
    }
}
