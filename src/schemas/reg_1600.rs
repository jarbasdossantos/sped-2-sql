// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        tot_credito -> Nullable<Text>,
        tot_debito -> Nullable<Text>,
    }
}
