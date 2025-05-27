// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0005 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        fantasia -> Nullable<Text>,
        cep -> Nullable<Text>,
        endereco -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
        fone -> Nullable<Text>,
        fax -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
