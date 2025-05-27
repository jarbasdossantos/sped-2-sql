// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_apur_icms -> Nullable<Text>,
        descr_compl_out_apur -> Nullable<Text>,
    }
}
