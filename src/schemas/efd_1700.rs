// @generated automatically by Diesel CLI.

diesel::table! {
    efd_1700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        pr_rec_ret -> Nullable<Text>,
        vl_ret_apu -> Nullable<Text>,
        vl_ret_ded -> Nullable<Text>,
        vl_ret_per -> Nullable<Text>,
        vl_ret_dcomp -> Nullable<Text>,
        sld_ret -> Nullable<Text>,
    }
}
