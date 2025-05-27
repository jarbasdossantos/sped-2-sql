// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c172 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        aliq_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
    }
}
