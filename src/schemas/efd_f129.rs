// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f129 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}
