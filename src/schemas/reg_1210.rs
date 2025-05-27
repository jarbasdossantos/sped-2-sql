// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        tipo_util -> Nullable<Text>,
        nr_doc -> Nullable<Text>,
        vl_cred_util -> Nullable<Text>,
    }
}
