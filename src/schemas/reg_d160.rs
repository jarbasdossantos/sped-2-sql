// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d160 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        despacho -> Nullable<Text>,
        cnpj_cpf_rem -> Nullable<Text>,
        ie_rem -> Nullable<Text>,
        cod_mun_ori -> Nullable<Text>,
        cnpj_cpf_dest -> Nullable<Text>,
        ie_dest -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
    }
}
