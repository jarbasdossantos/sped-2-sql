// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d180 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_seq -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cnpj_cpf_emit -> Nullable<Text>,
        uf_emit -> Nullable<Text>,
        ie_emit -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cnpj_cpf_tom -> Nullable<Text>,
        uf_tom -> Nullable<Text>,
        ie_tom -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
    }
}
