// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k250 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_prod -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
    }
}
