// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        ind_frt_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        vl_liq_frt -> Nullable<Text>,
        vl_sec_cat -> Nullable<Text>,
        vl_desp -> Nullable<Text>,
        vl_pedg -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}
