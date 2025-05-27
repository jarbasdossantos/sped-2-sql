// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis_tot -> Nullable<Text>,
        vl_bc_pis_cum -> Nullable<Text>,
        vl_bc_pis_nc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        quant_bc_pis_tot -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        desc_cred -> Nullable<Text>,
    }
}
