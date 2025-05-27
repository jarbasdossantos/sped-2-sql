// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c165 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        cod_aut -> Nullable<Text>,
        nr_passe -> Nullable<Text>,
        hora -> Nullable<Text>,
        temper -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
        nom_mot -> Nullable<Text>,
        cpf -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}
