// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        dt_ret -> Nullable<Text>,
        vl_bc_ret -> Nullable<Text>,
        vl_ret -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        ind_nat_rec -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_ret_pis -> Nullable<Text>,
        vl_ret_cofins -> Nullable<Text>,
        ind_dec -> Nullable<Text>,
    }
}
