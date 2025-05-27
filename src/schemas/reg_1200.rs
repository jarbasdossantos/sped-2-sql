// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        sld_cred -> Nullable<Text>,
        cred_apr -> Nullable<Text>,
        cred_receb -> Nullable<Text>,
        cred_util -> Nullable<Text>,
        sld_cred_fim -> Nullable<Text>,
    }
}
