// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        descr_ant_item -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fim -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
    }
}
