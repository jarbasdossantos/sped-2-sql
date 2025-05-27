// @generated automatically by Diesel CLI.

diesel::table! {
    efd_m600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_cont_nc_per -> Nullable<Text>,
        vl_tot_cred_desc -> Nullable<Text>,
        vl_tot_cred_desc_ant -> Nullable<Text>,
        vl_tot_cont_nc_dev -> Nullable<Text>,
        vl_ret_nc -> Nullable<Text>,
        vl_out_ded_nc -> Nullable<Text>,
        vl_cont_nc_rec -> Nullable<Text>,
        vl_tot_cont_cum_per -> Nullable<Text>,
        vl_ret_cum -> Nullable<Text>,
        vl_out_ded_cum -> Nullable<Text>,
        vl_cont_cum_rec -> Nullable<Text>,
        vl_tot_cont_rec -> Nullable<Text>,
    }
}
