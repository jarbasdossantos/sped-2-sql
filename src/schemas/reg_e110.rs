// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_debitos -> Nullable<Text>,
        vl_aj_debitos -> Nullable<Text>,
        vl_tot_aj_debitos -> Nullable<Text>,
        vl_estornos_cred -> Nullable<Text>,
        vl_tot_creditos -> Nullable<Text>,
        vl_aj_creditos -> Nullable<Text>,
        vl_tot_aj_creditos -> Nullable<Text>,
        vl_estornos_deb -> Nullable<Text>,
        vl_sld_credor_ant -> Nullable<Text>,
        vl_sld_apurado -> Nullable<Text>,
        vl_tot_ded -> Nullable<Text>,
        vl_icms_recolher -> Nullable<Text>,
        vl_sld_credor_transportar -> Nullable<Text>,
        deb_esp -> Nullable<Text>,
    }
}
