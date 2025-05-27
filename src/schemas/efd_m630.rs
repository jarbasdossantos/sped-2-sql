// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m630 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_vend -> Nullable<Text>,
        vl_nao_receb -> Nullable<Text>,
        vl_cont_dif -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
    }
}
