// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c173 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        lote_med -> Nullable<Text>,
        qtd_item -> Nullable<Text>,
        dt_fab -> Nullable<Text>,
        dt_val -> Nullable<Text>,
        ind_med -> Nullable<Text>,
        tp_prod -> Nullable<Text>,
        vl_tab_max -> Nullable<Text>,
    }
}
