// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_disp -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        num_aut -> Nullable<Text>,
    }
}
