// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ident_bem_imob -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        ind_util_bem_imob -> Nullable<Text>,
        vl_oper_dep -> Nullable<Text>,
        parc_oper_nao_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_bem_imob -> Nullable<Text>,
    }
}
