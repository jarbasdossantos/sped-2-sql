// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c174 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_arm -> Nullable<Text>,
        num_arm -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
    }
}
