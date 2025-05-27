// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        ind_tit -> Nullable<Text>,
        desc_tit -> Nullable<Text>,
        num_tit -> Nullable<Text>,
        qtd_parc -> Nullable<Text>,
        vl_tit -> Nullable<Text>,
    }
}
