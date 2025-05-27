// @generated automatically by Diesel CLI.

diesel::table! {
    reg_c112 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_da -> Nullable<Text>,
        uf -> Nullable<Text>,
        num_da -> Nullable<Text>,
        cod_aut -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        dt_pgto -> Nullable<Text>,
    }
}
