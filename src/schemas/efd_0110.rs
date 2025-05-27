// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inc_trib -> Nullable<Text>,
        ind_apro_cred -> Nullable<Text>,
        cod_tipo_cont -> Nullable<Text>,
        ind_reg_cum -> Nullable<Text>,
    }
}
