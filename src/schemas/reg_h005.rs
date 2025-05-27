// @generated automatically by Diesel CLI.

diesel::table! {
    reg_h005 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_inv -> Nullable<Text>,
        vl_inv -> Nullable<Text>,
        mot_inv -> Nullable<Text>,
    }
}
