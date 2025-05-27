// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e520 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_sd_ant_ipi -> Nullable<Text>,
        vl_deb_ipi -> Nullable<Text>,
        vl_cred_ipi -> Nullable<Text>,
        vl_od_ipi -> Nullable<Text>,
        vl_oc_ipi -> Nullable<Text>,
        vl_sc_ipi -> Nullable<Text>,
        vl_sd_ipi -> Nullable<Text>,
    }
}
