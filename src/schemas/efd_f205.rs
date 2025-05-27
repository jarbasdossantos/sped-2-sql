// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_inc_acum_ant -> Nullable<Text>,
        vl_cus_inc_per_esc -> Nullable<Text>,
        vl_cus_inc_acum -> Nullable<Text>,
        vl_exc_bc_cus_inc_acum -> Nullable<Text>,
        vl_bc_cus_inc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis_acum -> Nullable<Text>,
        vl_cred_pis_desc_ant -> Nullable<Text>,
        vl_cred_pis_desc -> Nullable<Text>,
        vl_cred_pis_desc_fut -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins_acum -> Nullable<Text>,
        vl_cred_cofins_desc_ant -> Nullable<Text>,
        vl_cred_cofins_desc -> Nullable<Text>,
        vl_cred_cofins_desc_fut -> Nullable<Text>,
    }
}
