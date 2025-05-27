// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
