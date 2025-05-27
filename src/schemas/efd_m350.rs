// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_fol -> Nullable<Text>,
        vl_exc_bc -> Nullable<Text>,
        vl_tot_bc -> Nullable<Text>,
        aliq_pis_fol -> Nullable<Text>,
        vl_tot_cont_fol -> Nullable<Text>,
    }
}
