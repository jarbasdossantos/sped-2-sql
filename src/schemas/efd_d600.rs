// @generated automatically by Diesel CLI.

diesel::table! {
    efd_d600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}
