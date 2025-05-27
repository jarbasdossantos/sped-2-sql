// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_est -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        ind_est -> Nullable<Text>,
        cod_part -> Nullable<Text>,
    }
}
