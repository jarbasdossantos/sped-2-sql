// @generated automatically by Diesel CLI.

diesel::table! {
    efd_1500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_apu_cred -> Nullable<Text>,
        orig_cred -> Nullable<Text>,
        cnpj_suc -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_apu -> Nullable<Text>,
        vl_cred_ext_apu -> Nullable<Text>,
        vl_tot_cred_apu -> Nullable<Text>,
        vl_cred_desc_pa_ant -> Nullable<Text>,
        vl_cred_per_pa_ant -> Nullable<Text>,
        vl_cred_dcomp_pa_ant -> Nullable<Text>,
        sd_cred_disp_efd -> Nullable<Text>,
        vl_cred_desc_efd -> Nullable<Text>,
        vl_cred_per_efd -> Nullable<Text>,
        vl_cred_dcomp_efd -> Nullable<Text>,
        vl_cred_trans -> Nullable<Text>,
        vl_cred_out -> Nullable<Text>,
        sld_cred_fim -> Nullable<Text>,
    }
}
