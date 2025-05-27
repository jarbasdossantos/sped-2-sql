// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1391 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_registro -> Nullable<Text>,
        qtd_moid -> Nullable<Text>,
        estq_ini -> Nullable<Text>,
        qtd_produz -> Nullable<Text>,
        ent_anid_hid -> Nullable<Text>,
        outr_entr -> Nullable<Text>,
        perda -> Nullable<Text>,
        cons -> Nullable<Text>,
        sai_ani_hid -> Nullable<Text>,
        saidas -> Nullable<Text>,
        estq_fin -> Nullable<Text>,
        estq_ini_mel -> Nullable<Text>,
        prod_dia_mel -> Nullable<Text>,
        util_mel -> Nullable<Text>,
        prod_alc_mel -> Nullable<Text>,
        obs -> Nullable<Text>,
    }
}
