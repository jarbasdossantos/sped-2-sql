// @generated automatically by Diesel CLI.

diesel::table! {
    efd_f700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_ori_ded -> Nullable<Text>,
        ind_nat_ded -> Nullable<Text>,
        vl_ded_pis -> Nullable<Text>,
        vl_ded_cofins -> Nullable<Text>,
        vl_bc_oper -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}
