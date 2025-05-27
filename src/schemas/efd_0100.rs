// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nome -> Nullable<Text>,
        cpf -> Nullable<Text>,
        crc -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cep -> Nullable<Text>,
        end -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
        fone -> Nullable<Text>,
        fax -> Nullable<Text>,
        email -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
    }
}
