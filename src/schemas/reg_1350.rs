// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        serie -> Nullable<Text>,
        fabricante -> Nullable<Text>,
        modelo -> Nullable<Text>,
        tipo_medicao -> Nullable<Text>,
    }
}
