// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        ind_cred_ori -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_cred -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        vl_cred_disp -> Nullable<Text>,
        ind_desc_cred -> Nullable<Text>,
        vl_cred_desc -> Nullable<Text>,
        sld_cred -> Nullable<Text>,
    }
}
