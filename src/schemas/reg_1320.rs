// @generated automatically by Diesel CLI.

diesel::table! {
    reg_1320 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_bico -> Nullable<Text>,
        nr_interv -> Nullable<Text>,
        mot_interv -> Nullable<Text>,
        nom_interv -> Nullable<Text>,
        cnpj_interv -> Nullable<Text>,
        cpf_interv -> Nullable<Text>,
        val_fecha -> Nullable<Text>,
        val_abert -> Nullable<Text>,
        vol_aferi -> Nullable<Text>,
        vol_vendas -> Nullable<Text>,
    }
}
