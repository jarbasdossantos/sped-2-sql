// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}
