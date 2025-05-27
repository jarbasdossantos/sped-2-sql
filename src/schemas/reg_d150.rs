// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        viagem -> Nullable<Text>,
        ind_tfa -> Nullable<Text>,
        vl_peso_tx -> Nullable<Text>,
        vl_tx_terr -> Nullable<Text>,
        vl_tx_red -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_tx_adv -> Nullable<Text>,
    }
}
