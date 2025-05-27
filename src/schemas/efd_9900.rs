// @generated automatically by Diesel CLI.

diesel::table! {
    efd_9900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        reg_blc -> Nullable<Text>,
        qtd_reg_blc -> Nullable<Text>,
    }
}
