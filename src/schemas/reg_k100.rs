// @generated automatically by Diesel CLI.

diesel::table! {
    reg_k100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}
