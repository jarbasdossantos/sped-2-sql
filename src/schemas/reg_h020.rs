// @generated automatically by Diesel CLI.

diesel::table! {
    reg_h020 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        bl_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}
