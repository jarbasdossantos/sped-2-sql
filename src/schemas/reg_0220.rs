// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid_conv -> Nullable<Text>,
        fat_conv -> Nullable<Text>,
    }
}
