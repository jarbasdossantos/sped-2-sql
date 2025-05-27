// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0000 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        tipo_escrit -> Nullable<Text>,
        ind_sit_esp -> Nullable<Text>,
        num_rec_anterior -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        suframa -> Nullable<Text>,
        ind_nat_pj -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
    }
}
