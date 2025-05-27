// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}
