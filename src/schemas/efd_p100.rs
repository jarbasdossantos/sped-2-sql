// @generated automatically by Diesel CLI.

diesel::table! {
    efd_p100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fim -> Nullable<Text>,
        vl_rec_tot_est -> Nullable<Text>,
        cod_ativ_econ -> Nullable<Text>,
        vl_rec_ativ_estab -> Nullable<Text>,
        vl_exc -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
        aliq_cont -> Nullable<Text>,
        vl_cont_apu -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}
