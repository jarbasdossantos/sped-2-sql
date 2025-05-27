// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c179 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        bc_st_orig_dest -> Nullable<Text>,
        icms_st_rep -> Nullable<Text>,
        icms_st_compl -> Nullable<Text>,
        bc_ret -> Nullable<Text>,
        icms_ret -> Nullable<Text>,
    }
}
