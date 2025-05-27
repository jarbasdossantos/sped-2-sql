// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c460 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cpf_cnpj -> Nullable<Text>,
        nome_adq -> Nullable<Text>,
    }
}
