// @generated automatically by Diesel CLI.

diesel::table! {
    efd_i300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comp -> Nullable<Text>,
        vl_comp -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
