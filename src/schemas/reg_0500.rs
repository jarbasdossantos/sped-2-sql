// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_nat_cc -> Nullable<Text>,
        ind_cta -> Nullable<Text>,
        nivel -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        nome_cta -> Nullable<Text>,
    }
}
