// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c481 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
