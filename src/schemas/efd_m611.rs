// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m611 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_tip_coop -> Nullable<Text>,
        vl_bc_cont_ant_exc_coop -> Nullable<Text>,
        vl_exc_coop_ger -> Nullable<Text>,
        vl_exc_esp_coop -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
    }
}
