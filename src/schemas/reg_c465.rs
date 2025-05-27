// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c465 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        num_ccf -> Nullable<Text>,
    }
}
