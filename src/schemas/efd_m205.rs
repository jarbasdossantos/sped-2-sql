// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        vl_debito -> Nullable<Text>,
    }
}
