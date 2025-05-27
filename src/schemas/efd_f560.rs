// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f560 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_comp -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}
