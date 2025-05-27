// @generated automatically by Diesel CLI.

diesel::table! {
    efd_p110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_det -> Nullable<Text>,
        det_valor -> Nullable<Text>,
        inf_compl -> Nullable<Text>,
    }
}
