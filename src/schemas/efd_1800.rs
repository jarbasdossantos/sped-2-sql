// @generated automatically by Diesel CLI.

diesel::table! {
    efd_1800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        inc_imob -> Nullable<Text>,
        rec_receb_ret -> Nullable<Text>,
        rec_fin_ret -> Nullable<Text>,
        bc_ret -> Nullable<Text>,
        aliq_ret -> Nullable<Text>,
        vl_rec_uni -> Nullable<Text>,
        dt_rec_uni -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
    }
}
