// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}
