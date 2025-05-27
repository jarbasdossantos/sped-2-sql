// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_rec_brt -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_cont_apur -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cont_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        vl_cont_per -> Nullable<Text>,
    }
}
