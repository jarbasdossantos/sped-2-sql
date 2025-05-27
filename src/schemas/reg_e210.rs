// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov_st -> Nullable<Text>,
        vl_sld_cred_ant_st -> Nullable<Text>,
        vl_devol_st -> Nullable<Text>,
        vl_ressarc_st -> Nullable<Text>,
        vl_out_cred_st -> Nullable<Text>,
        vl_aj_creditos_st -> Nullable<Text>,
        vl_retencao_st -> Nullable<Text>,
        vl_out_deb_st -> Nullable<Text>,
        vl_aj_debitos_st -> Nullable<Text>,
        vl_sld_dev_ant_st -> Nullable<Text>,
        vl_deducoes_st -> Nullable<Text>,
        vl_icms_recol_st -> Nullable<Text>,
        vl_sld_cred_st_transportar -> Nullable<Text>,
        deb_esp_st -> Nullable<Text>,
    }
}
