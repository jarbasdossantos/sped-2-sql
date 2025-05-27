// @generated automatically by Diesel CLI.

diesel::table! {
    efd_a100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfse -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_exe_serv -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        vl_pis_ret -> Nullable<Text>,
        vl_cofins_ret -> Nullable<Text>,
        vl_iss -> Nullable<Text>,
    }
}
