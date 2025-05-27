// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
