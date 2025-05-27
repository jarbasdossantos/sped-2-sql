// @generated automatically by Diesel CLI.

diesel::table! {
    reg_0300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ind_bem -> Nullable<Text>,
        ident_merc -> Nullable<Text>,
        descr_item -> Nullable<Text>,
        cod_prnc -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        nr_parc -> Nullable<Text>,
    }
}
