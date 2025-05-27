// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_even -> Nullable<Text>,
        dt_even -> Nullable<Text>,
        cnpj_suced -> Nullable<Text>,
        pa_cont_cred -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_pis -> Nullable<Text>,
        vl_cred_cofins -> Nullable<Text>,
        per_cred_cis -> Nullable<Text>,
    }
}
