// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m410 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_rec -> Nullable<Text>,
        vl_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}
