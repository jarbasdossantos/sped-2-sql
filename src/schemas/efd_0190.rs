// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid -> Nullable<Text>,
        descr -> Nullable<Text>,
    }
}
