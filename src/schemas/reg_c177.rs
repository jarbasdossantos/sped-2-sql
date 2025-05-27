// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c177 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_selo_ipi -> Nullable<Text>,
        qt_selo_ipi -> Nullable<Text>,
    }
}
