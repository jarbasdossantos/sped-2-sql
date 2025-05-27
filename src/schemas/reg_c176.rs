// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c176 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod_ult_e -> Nullable<Text>,
        num_doc_ult_e -> Nullable<Text>,
        ser_ult_e -> Nullable<Text>,
        dt_ult_e -> Nullable<Text>,
        cod_part_ult_e -> Nullable<Text>,
        quant_ult_e -> Nullable<Text>,
        vl_unit_ult_e -> Nullable<Text>,
        vl_unit_bc_st -> Nullable<Text>,
    }
}
