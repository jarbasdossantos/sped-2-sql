// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k235 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_saida -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        cod_ins_subst -> Nullable<Text>,
    }
}
