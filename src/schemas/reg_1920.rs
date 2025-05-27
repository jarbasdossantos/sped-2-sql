// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1920 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_transf_debitos_oa -> Nullable<Text>,
        vl_tot_aj_debitos_oa -> Nullable<Text>,
        vl_estornos_cred_oa -> Nullable<Text>,
        vl_tot_transf_creditos_oa -> Nullable<Text>,
        vl_tot_aj_creditos_oa -> Nullable<Text>,
        vl_estornos_deb_oa -> Nullable<Text>,
        vl_sld_credor_ant_oa -> Nullable<Text>,
        vl_sld_apurado_oa -> Nullable<Text>,
        vl_tot_ded -> Nullable<Text>,
        vl_icms_recolher_oa -> Nullable<Text>,
        vl_sld_credor_transp_oa -> Nullable<Text>,
        deb_esp_oa -> Nullable<Text>,
    }
}
