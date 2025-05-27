// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1310 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        estq_abert -> Nullable<Text>,
        vol_entr -> Nullable<Text>,
        vol_disp -> Nullable<Text>,
        vol_saidas -> Nullable<Text>,
        estq_escr -> Nullable<Text>,
        val_aj_perda -> Nullable<Text>,
        val_aj_ganho -> Nullable<Text>,
        fech_fisico -> Nullable<Text>,
    }
}
