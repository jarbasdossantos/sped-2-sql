// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}
