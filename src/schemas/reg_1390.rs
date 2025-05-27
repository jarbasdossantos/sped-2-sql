// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1390 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_prod -> Nullable<Text>,
    }
}
