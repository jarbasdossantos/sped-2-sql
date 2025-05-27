// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1926 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        mes_ref -> Nullable<Text>,
    }
}
