// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d301 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}
