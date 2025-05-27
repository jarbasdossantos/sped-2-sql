// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1921 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Text>,
    }
}
