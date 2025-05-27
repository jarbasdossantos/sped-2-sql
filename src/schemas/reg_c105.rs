// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        oper -> Nullable<Text>,
        cod_uf -> Nullable<Text>,
    }
}
