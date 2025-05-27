// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_orc -> Nullable<Text>,
        vl_exc -> Nullable<Text>,
        vl_cus_orc_aju -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis_util -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins_util -> Nullable<Text>,
    }
}
