// @generated automatically by Diesel CLI.

diesel::table! {
    efd_p200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_ref -> Nullable<Text>,
        vl_tot_cont_apu -> Nullable<Text>,
        vl_tot_aj_reduc -> Nullable<Text>,
        vl_tot_aj_acres -> Nullable<Text>,
        vl_tot_cont_dev -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
    }
}
