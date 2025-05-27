// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e115 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf_adic -> Nullable<Text>,
        vl_inf_adic -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
    }
}
