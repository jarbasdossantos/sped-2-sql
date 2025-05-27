// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1910 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}
