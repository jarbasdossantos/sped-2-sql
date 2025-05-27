// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1360 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_lacre -> Nullable<Text>,
        dat_aplicacao -> Nullable<Text>,
    }
}
