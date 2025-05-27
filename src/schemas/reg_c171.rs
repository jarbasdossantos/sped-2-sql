// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c171 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        qtde -> Nullable<Text>,
    }
}
