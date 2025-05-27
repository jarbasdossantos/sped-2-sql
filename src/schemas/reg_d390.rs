// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d390 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        aliq_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}
