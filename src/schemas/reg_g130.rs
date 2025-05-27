// @generated automatically by Diesel CLI.

diesel::table! {
    reg_g130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        serie -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe_cte -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}
