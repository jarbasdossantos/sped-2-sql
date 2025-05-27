// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        nome -> Nullable<Text>,
        cod_pais -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        suframa -> Nullable<Text>,
        end -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
    }
}
