// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        qtd_bilh -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}
