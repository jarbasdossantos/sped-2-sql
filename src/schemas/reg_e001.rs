// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}
