// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_apur -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}
