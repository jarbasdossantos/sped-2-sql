// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cons -> Nullable<Text>,
        vl_forn -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}
