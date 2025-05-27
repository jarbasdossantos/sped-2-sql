// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        rec_bru_ncum_trib_mi -> Nullable<Text>,
        rec_bru_ncum_nt_mi -> Nullable<Text>,
        rec_bru_ncum_exp -> Nullable<Text>,
        rec_bru_cum -> Nullable<Text>,
        rec_bru_total -> Nullable<Text>,
    }
}
