// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_bico -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
    }
}
