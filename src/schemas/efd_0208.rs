// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0208 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tab -> Nullable<Text>,
        cod_gru -> Nullable<Text>,
        marca_com -> Nullable<Text>,
    }
}
