// @generated automatically by Diesel CLI.

diesel::table! {
    efd_1010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        id_sec_jud -> Nullable<Text>,
        id_vara -> Nullable<Text>,
        ind_nat_acao -> Nullable<Text>,
        desc_dec_jud -> Nullable<Text>,
        dt_sent_jud -> Nullable<Text>,
    }
}
