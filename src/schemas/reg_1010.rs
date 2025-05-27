// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_exp -> Nullable<Text>,
        ind_ccrf -> Nullable<Text>,
        ind_comb -> Nullable<Text>,
        ind_usina -> Nullable<Text>,
        ind_va -> Nullable<Text>,
        ind_ee -> Nullable<Text>,
        ind_cart -> Nullable<Text>,
        ind_form -> Nullable<Text>,
        ind_aer -> Nullable<Text>,
    }
}
