// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d697 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
    }
}
