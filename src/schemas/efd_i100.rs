// @generated automatically by Diesel CLI.

diesel::table! {
    efd_i100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_fin -> Nullable<Text>,
        cst -> Nullable<Text>,
        vl_tot_ded_ger -> Nullable<Text>,
        vl_tot_ded_esp -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
