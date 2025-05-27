// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d161 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_carga -> Nullable<Text>,
        cnpj_cpf_col -> Nullable<Text>,
        ie_col -> Nullable<Text>,
        cod_mun_col -> Nullable<Text>,
        cnpj_cpf_entg -> Nullable<Text>,
        ie_entg -> Nullable<Text>,
        cod_mun_entg -> Nullable<Text>,
    }
}
