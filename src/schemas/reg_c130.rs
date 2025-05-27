// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
        vl_bc_irrf -> Nullable<Text>,
        vl_irrf -> Nullable<Text>,
        vl_bc_prev -> Nullable<Text>,
        vl_prev -> Nullable<Text>,
    }
}
