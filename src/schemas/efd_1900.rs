// @generated automatically by Diesel CLI.

diesel::table! {
    efd_1900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        quant_doc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
