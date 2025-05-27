// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d162 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
    }
}
