// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0305 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        func -> Nullable<Text>,
        vida_util -> Nullable<Text>,
    }
}
