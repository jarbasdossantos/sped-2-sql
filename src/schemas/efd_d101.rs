// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d101 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
