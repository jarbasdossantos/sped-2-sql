// @generated automatically by Diesel CLI.

diesel::table! {
    reg_d350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
    }
}
