// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}
