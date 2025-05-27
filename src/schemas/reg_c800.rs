// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_cfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_cfe -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        vl_out_da -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis_st -> Nullable<Text>,
        vl_cofins_st -> Nullable<Text>,
    }
}
