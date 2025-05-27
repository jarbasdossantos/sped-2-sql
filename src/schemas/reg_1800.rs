// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_carga -> Nullable<Text>,
        vl_pass -> Nullable<Text>,
        vl_fat -> Nullable<Text>,
        ind_rat -> Nullable<Text>,
        vl_icms_ant -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms_apur -> Nullable<Text>,
        vl_bc_icms_apur -> Nullable<Text>,
        vl_dif -> Nullable<Text>,
    }
}
