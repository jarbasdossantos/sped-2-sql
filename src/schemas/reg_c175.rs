// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c175 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_veic_oper -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        chassi_veic -> Nullable<Text>,
    }
}
