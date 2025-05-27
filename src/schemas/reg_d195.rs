// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d195 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}
