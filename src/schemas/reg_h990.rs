// @generated automatically by Diesel CLI.

diesel::table! {
    reg_h990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_h -> Nullable<Text>,
    }
}
