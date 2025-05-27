// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        mun -> Nullable<Text>,
        valor -> Nullable<Text>,
    }
}
