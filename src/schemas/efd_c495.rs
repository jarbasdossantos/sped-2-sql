// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c495 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
