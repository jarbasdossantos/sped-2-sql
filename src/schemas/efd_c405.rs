// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c405 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
    }
}
