// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_cont_apur_difer -> Nullable<Text>,
        nat_bc_cred_desc -> Nullable<Text>,
        vl_cred_desc_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        per_apur -> Nullable<Text>,
        dt_receb -> Nullable<Text>,
    }
}
