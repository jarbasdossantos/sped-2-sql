// @generated automatically by Diesel CLI.

diesel::table! {
    reg_g126 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Text>,
        vl_trib_oc -> Nullable<Text>,
        vl_total -> Nullable<Text>,
        ind_per_sai -> Nullable<Text>,
        vl_parc_aprop -> Nullable<Text>,
    }
}
