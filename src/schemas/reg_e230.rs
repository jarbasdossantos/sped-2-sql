// @generated automatically by Diesel CLI.

diesel::table! {
    reg_e230 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_da -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}
