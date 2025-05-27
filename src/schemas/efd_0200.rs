// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_item -> Nullable<Text>,
        cod_barra -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
        unid_inv -> Nullable<Text>,
        tipo_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        cod_gen -> Nullable<Text>,
        cod_lst -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
    }
}
