// @generated automatically by Diesel CLI.

diesel::table! {
    reg_g125 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ind_bem -> Nullable<Text>,
        dt_mov -> Nullable<Text>,
        tipo_mov -> Nullable<Text>,
        vl_imob_icms_op -> Nullable<Text>,
        vl_imob_icms_st -> Nullable<Text>,
        vl_imob_icms_frt -> Nullable<Text>,
        vl_imob_icms_dif -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Text>,
    }
}
