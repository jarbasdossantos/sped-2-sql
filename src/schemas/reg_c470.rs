// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c470 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}
