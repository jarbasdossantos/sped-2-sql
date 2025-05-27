// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        ind_veic -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        ind_nav -> Nullable<Text>,
        viagem -> Nullable<Text>,
        vl_frt_liq -> Nullable<Text>,
        vl_desp_port -> Nullable<Text>,
        vl_desp_car_desc -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt_brt -> Nullable<Text>,
        vl_frt_mm -> Nullable<Text>,
    }
}
