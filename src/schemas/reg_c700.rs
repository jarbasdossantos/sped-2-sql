// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        nro_ord_ini -> Nullable<Text>,
        nro_ord_fin -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        nom_mest -> Nullable<Text>,
        chv_cod_dig -> Nullable<Text>,
    }
}
