// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_doc -> Nullable<Text>,
        nro_de -> Nullable<Text>,
        dt_de -> Nullable<Text>,
        nat_exp -> Nullable<Text>,
        nro_re -> Nullable<Text>,
        dt_re -> Nullable<Text>,
        chc_emb -> Nullable<Text>,
        dt_chc -> Nullable<Text>,
        dt_avb -> Nullable<Text>,
        tp_chc -> Nullable<Text>,
        pais -> Nullable<Text>,
    }
}
