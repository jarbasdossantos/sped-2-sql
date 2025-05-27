// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0000 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        cod_fin -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        uf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        im -> Nullable<Text>,
        suframa -> Nullable<Text>,
        ind_perfil -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
    }
}
