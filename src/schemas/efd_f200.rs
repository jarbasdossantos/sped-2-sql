// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        unid_imob -> Nullable<Text>,
        ident_emp -> Nullable<Text>,
        desc_unid_imob -> Nullable<Text>,
        num_cont -> Nullable<Text>,
        cpf_cnpj_adqu -> Nullable<Text>,
        dt_oper -> Nullable<Text>,
        vl_tot_vend -> Nullable<Text>,
        vl_rec_acum -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        perc_rec_receb -> Nullable<Text>,
        ind_nat_emp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
