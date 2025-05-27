// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c116 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        num_cfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}
