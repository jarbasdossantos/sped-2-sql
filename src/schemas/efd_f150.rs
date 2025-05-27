// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_tot_est -> Nullable<Text>,
        est_imp -> Nullable<Text>,
        vl_bc_est -> Nullable<Text>,
        vl_bc_men_est -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins -> Nullable<Text>,
        desc_est -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
