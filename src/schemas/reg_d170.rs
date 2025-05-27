// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d170 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        otm -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_liq_frt -> Nullable<Text>,
        vl_gris -> Nullable<Text>,
        vl_pdg -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}
