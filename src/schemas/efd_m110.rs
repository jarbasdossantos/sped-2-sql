// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}
