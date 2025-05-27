// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_out -> Nullable<Text>,
    }
}
