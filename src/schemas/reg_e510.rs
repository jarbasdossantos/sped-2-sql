// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        vl_cont_ipi -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
    }
}
