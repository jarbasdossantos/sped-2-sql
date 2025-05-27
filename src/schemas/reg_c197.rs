// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c197 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_outros -> Nullable<Text>,
    }
}
