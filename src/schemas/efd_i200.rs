// @generated automatically by Diesel CLI.

diesel::table! {
    efd_i200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_det -> Nullable<Text>,
        vl_det -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
