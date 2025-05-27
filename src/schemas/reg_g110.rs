// @generated automatically by Diesel CLI.

diesel::table! {
    reg_g110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        saldo_in_icms -> Nullable<Text>,
        som_parc -> Nullable<Text>,
        vl_trib_exp -> Nullable<Text>,
        vl_total -> Nullable<Text>,
        ind_per_sai -> Nullable<Text>,
        icms_aprop -> Nullable<Text>,
        som_icms_oc -> Nullable<Text>,
    }
}
