// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c114 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}
