// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0015 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf_st -> Nullable<Text>,
        ie_st -> Nullable<Text>,
    }
}
