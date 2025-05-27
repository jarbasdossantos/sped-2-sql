// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_seg -> Nullable<Text>,
        vl_out_desp -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
