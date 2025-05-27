// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0145 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inc_trib -> Nullable<Text>,
        vl_rec_tot -> Nullable<Text>,
        vl_rec_ativ -> Nullable<Text>,
        vl_rec_demais_ativ -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}
