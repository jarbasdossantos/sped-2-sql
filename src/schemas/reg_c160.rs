// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c160 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}
