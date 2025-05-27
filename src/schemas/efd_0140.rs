// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_est -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        im -> Nullable<Text>,
        suframa -> Nullable<Text>,
    }
}
