// @generated automatically by Diesel CLI.

diesel::table! {
    efd_a120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_serv -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        vl_pis_imp -> Nullable<Text>,
        dt_pag_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        vl_cofins_imp -> Nullable<Text>,
        dt_pag_cofins -> Nullable<Text>,
        loc_exe_serv -> Nullable<Text>,
    }
}
