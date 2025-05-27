// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c495 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_canc -> Nullable<Text>,
        vl_acmo -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_isen -> Nullable<Text>,
        vl_nt -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
    }
}
