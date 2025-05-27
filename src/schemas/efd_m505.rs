// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m505 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins_tot -> Nullable<Text>,
        vl_bc_cofins_cum -> Nullable<Text>,
        vl_bc_cofins_nc -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        quant_bc_cofins_tot -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        desc_cred -> Nullable<Text>,
    }
}
