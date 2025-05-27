// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_oper -> Nullable<Text>,
        vl_oper -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_doc_oper -> Nullable<Text>,
    }
}
