// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0035 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_scp -> Nullable<Text>,
        nome_scp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
