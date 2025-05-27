// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c178 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cl_enq -> Nullable<Text>,
        vl_unid -> Nullable<Text>,
        quant_pad -> Nullable<Text>,
    }
}
