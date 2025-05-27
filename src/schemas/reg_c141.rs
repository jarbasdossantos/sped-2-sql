// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c141 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        vl_parc -> Nullable<Text>,
    }
}
