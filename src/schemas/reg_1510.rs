// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        aliq_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
