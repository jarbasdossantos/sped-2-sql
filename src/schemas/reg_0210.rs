// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item_comp -> Nullable<Text>,
        qtd_comp -> Nullable<Text>,
        perda -> Nullable<Text>,
    }
}
