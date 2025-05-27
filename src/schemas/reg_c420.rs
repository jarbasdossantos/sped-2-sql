// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c420 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tot_par -> Nullable<Text>,
        vlr_acum_tot -> Nullable<Text>,
        nr_tot -> Nullable<Text>,
        descr_nr_tot -> Nullable<Text>,
    }
}
