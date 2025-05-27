// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        mes_dispensa -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
