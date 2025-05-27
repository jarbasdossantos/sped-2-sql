// @generated automatically by Diesel CLI.

diesel::table! {
    reg_h010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        unid -> Nullable<Text>,
        qtd -> Nullable<Text>,
        vl_unit -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        ind_prop -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        vl_item_ir -> Nullable<Text>,
    }
}
