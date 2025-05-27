// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0175 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        nr_campo -> Nullable<Text>,
        cont_ant -> Nullable<Text>,
    }
}
