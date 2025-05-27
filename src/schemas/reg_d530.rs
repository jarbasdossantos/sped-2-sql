// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d530 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_serv -> Nullable<Text>,
        dt_ini_serv -> Nullable<Text>,
        dt_fin_serv -> Nullable<Text>,
        per_fiscal -> Nullable<Text>,
        cod_area -> Nullable<Text>,
        terminal -> Nullable<Text>,
    }
}
