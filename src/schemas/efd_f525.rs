// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f525 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_rec_det -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
