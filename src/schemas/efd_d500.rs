// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_a_p -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}
