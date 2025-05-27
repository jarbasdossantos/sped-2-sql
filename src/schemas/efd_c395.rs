// @generated automatically by Diesel CLI.

diesel::table! {
    efd_c395 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
    }
}
