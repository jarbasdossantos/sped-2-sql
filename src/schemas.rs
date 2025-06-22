// @generated automatically by Diesel CLI.

diesel::table! {
    efd_0000 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        tipo_escrit -> Nullable<Text>,
        ind_sit_esp -> Nullable<Text>,
        num_rec_anterior -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        suframa -> Nullable<Text>,
        ind_nat_pj -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0035 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_scp -> Nullable<Text>,
        nome_scp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nome -> Nullable<Text>,
        cpf -> Nullable<Text>,
        crc -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cep -> Nullable<Text>,
        end -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
        fone -> Nullable<Text>,
        fax -> Nullable<Text>,
        email -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inc_trib -> Nullable<Text>,
        ind_apro_cred -> Nullable<Text>,
        cod_tipo_cont -> Nullable<Text>,
        ind_reg_cum -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        rec_bru_ncum_trib_mi -> Nullable<Text>,
        rec_bru_ncum_nt_mi -> Nullable<Text>,
        rec_bru_ncum_exp -> Nullable<Text>,
        rec_bru_cum -> Nullable<Text>,
        rec_bru_total -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        mes_dispensa -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_est -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        im -> Nullable<Text>,
        suframa -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0145 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inc_trib -> Nullable<Text>,
        vl_rec_tot -> Nullable<Text>,
        vl_rec_ativ -> Nullable<Text>,
        vl_rec_demais_ativ -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        nome -> Nullable<Text>,
        cod_pais -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        suframa -> Nullable<Text>,
        end -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid -> Nullable<Text>,
        descr -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_item -> Nullable<Text>,
        cod_barra -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
        unid_inv -> Nullable<Text>,
        tipo_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        cod_gen -> Nullable<Text>,
        cod_lst -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        descr_ant_item -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fim -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0206 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comb -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0208 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tab -> Nullable<Text>,
        cod_gru -> Nullable<Text>,
        marca_com -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        descr_nat -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0450 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_nat_cc -> Nullable<Text>,
        ind_cta -> Nullable<Text>,
        nivel -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        nome_cta -> Nullable<Text>,
        cod_cta_ref -> Nullable<Text>,
        cnpj_est -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        ccus -> Nullable<Text>,
    }
}

diesel::table! {
    efd_0990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_0 -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        id_sec_jud -> Nullable<Text>,
        id_vara -> Nullable<Text>,
        ind_nat_acao -> Nullable<Text>,
        desc_dec_jud -> Nullable<Text>,
        dt_sent_jud -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1020 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_nat_acao -> Nullable<Text>,
        dt_dec_adm -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_apu_cred -> Nullable<Text>,
        orig_cred -> Nullable<Text>,
        cnpj_suc -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_apu -> Nullable<Text>,
        vl_cred_ext_apu -> Nullable<Text>,
        vl_tot_cred_apu -> Nullable<Text>,
        vl_cred_desc_pa_ant -> Nullable<Text>,
        vl_cred_per_pa_ant -> Nullable<Text>,
        vl_cred_dcomp_pa_ant -> Nullable<Text>,
        sd_cred_disp_efd -> Nullable<Text>,
        vl_cred_desc_efd -> Nullable<Text>,
        vl_cred_per_efd -> Nullable<Text>,
        vl_cred_dcomp_efd -> Nullable<Text>,
        vl_cred_trans -> Nullable<Text>,
        vl_cred_out -> Nullable<Text>,
        sld_cred_fim -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        pr_rec_ret -> Nullable<Text>,
        vl_ret_apu -> Nullable<Text>,
        vl_ret_ded -> Nullable<Text>,
        vl_ret_per -> Nullable<Text>,
        vl_ret_dcomp -> Nullable<Text>,
        sld_ret -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_apu_cred -> Nullable<Text>,
        orig_cred -> Nullable<Text>,
        cnpj_suc -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_apu -> Nullable<Text>,
        vl_cred_ext_apu -> Nullable<Text>,
        vl_tot_cred_apu -> Nullable<Text>,
        vl_cred_desc_pa_ant -> Nullable<Text>,
        vl_cred_per_pa_ant -> Nullable<Text>,
        vl_cred_dcomp_pa_ant -> Nullable<Text>,
        sd_cred_disp_efd -> Nullable<Text>,
        vl_cred_desc_efd -> Nullable<Text>,
        vl_cred_per_efd -> Nullable<Text>,
        vl_cred_dcomp_efd -> Nullable<Text>,
        vl_cred_trans -> Nullable<Text>,
        vl_cred_out -> Nullable<Text>,
        sld_cred_fim -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        pr_rec_ret -> Nullable<Text>,
        vl_ret_apu -> Nullable<Text>,
        vl_ret_ded -> Nullable<Text>,
        vl_ret_per -> Nullable<Text>,
        vl_ret_dcomp -> Nullable<Text>,
        sld_ret -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        inc_imob -> Nullable<Text>,
        rec_receb_ret -> Nullable<Text>,
        rec_fin_ret -> Nullable<Text>,
        bc_ret -> Nullable<Text>,
        aliq_ret -> Nullable<Text>,
        vl_rec_uni -> Nullable<Text>,
        dt_rec_uni -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        quant_doc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_1990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_1 -> Nullable<Text>,
    }
}

diesel::table! {
    efd_9001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

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

diesel::table! {
    efd_9990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_9 -> Nullable<Text>,
    }
}

diesel::table! {
    efd_9999 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfse -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_exe_serv -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        vl_pis_ret -> Nullable<Text>,
        vl_cofins_ret -> Nullable<Text>,
        vl_iss -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_serv -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        vl_pis_imp -> Nullable<Text>,
        dt_pag_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        vl_cofins_imp -> Nullable<Text>,
        dt_pag_cofins -> Nullable<Text>,
        loc_exe_serv -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a170 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
    }
}

diesel::table! {
    efd_a990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_a -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        ind_escri -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_e_s -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_abat_nt -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        ind_frt -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        vl_seg -> Nullable<Text>,
        vl_out_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        vl_pis_st -> Nullable<Text>,
        vl_cofins_st -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        vl_pis_imp -> Nullable<Text>,
        vl_cofins_imp -> Nullable<Text>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c170 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        aliq_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_apur -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        cod_enq -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Text>,
        aliq_ipi -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c180 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        vl_tot_item -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c181 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c185 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c188 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_ref_ini -> Nullable<Text>,
        dt_ref_fin -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        vl_tot_item -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c191 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj_cpf_part -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c195 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj_cpf_part -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c198 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c199 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        vl_pis_imp -> Nullable<Text>,
        vl_cofins_imp -> Nullable<Text>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c380 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c381 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c385 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c395 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c396 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c405 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c481 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c485 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c489 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c490 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c491 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c495 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c499 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_e_s -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c501 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c505 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c509 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cons -> Nullable<Text>,
        vl_forn -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c601 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c605 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c609 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c860 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        doc_ini -> Nullable<Text>,
        doc_fim -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c870 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c880 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c890 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_c990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_c -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_cte -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_a_p -> Nullable<Text>,
        tp_cte -> Nullable<Text>,
        chv_cte_ref -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        ind_frt -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_nt -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d101 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cfop -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d201 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d209 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cfop -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d309 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d359 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_a_p -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d501 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d505 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d509 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d601 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d605 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d609 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_d990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_d -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_oper -> Nullable<Text>,
        vl_oper -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_doc_oper -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ident_bem_imob -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        ind_util_bem_imob -> Nullable<Text>,
        vl_oper_dep -> Nullable<Text>,
        parc_oper_nao_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_bem_imob -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f129 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ident_bem_imob -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        ind_util_bem_imob -> Nullable<Text>,
        mes_oper_aquis -> Nullable<Text>,
        vl_oper_aquis -> Nullable<Text>,
        parc_oper_nao_bc_cred -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        ind_nr_parc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_bem_imob -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f139 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_tot_est -> Nullable<Text>,
        est_imp -> Nullable<Text>,
        vl_bc_est -> Nullable<Text>,
        vl_bc_men_est -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins -> Nullable<Text>,
        desc_est -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        unid_imob -> Nullable<Text>,
        ident_emp -> Nullable<Text>,
        desc_unid_imob -> Nullable<Text>,
        num_cont -> Nullable<Text>,
        cpf_cnpj_adqu -> Nullable<Text>,
        dt_oper -> Nullable<Text>,
        vl_tot_vend -> Nullable<Text>,
        vl_rec_acum -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        perc_rec_receb -> Nullable<Text>,
        ind_nat_emp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_inc_acum_ant -> Nullable<Text>,
        vl_cus_inc_per_esc -> Nullable<Text>,
        vl_cus_inc_acum -> Nullable<Text>,
        vl_exc_bc_cus_inc_acum -> Nullable<Text>,
        vl_bc_cus_inc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis_acum -> Nullable<Text>,
        vl_cred_pis_desc_ant -> Nullable<Text>,
        vl_cred_pis_desc -> Nullable<Text>,
        vl_cred_pis_desc_fut -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins_acum -> Nullable<Text>,
        vl_cred_cofins_desc_ant -> Nullable<Text>,
        vl_cred_cofins_desc -> Nullable<Text>,
        vl_cred_cofins_desc_fut -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_orc -> Nullable<Text>,
        vl_exc -> Nullable<Text>,
        vl_cus_orc_aju -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_cred_pis_util -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cred_cofins_util -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f211 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_caixa -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f509 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_caixa -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f519 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f525 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_rec_det -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f550 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_comp -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f559 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f560 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_comp -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f569 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

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

diesel::table! {
    efd_f800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_even -> Nullable<Text>,
        dt_even -> Nullable<Text>,
        cnpj_suced -> Nullable<Text>,
        pa_cont_cred -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_pis -> Nullable<Text>,
        vl_cred_cofins -> Nullable<Text>,
        per_cred_cis -> Nullable<Text>,
    }
}

diesel::table! {
    efd_f990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_f -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_fin -> Nullable<Text>,
        cst -> Nullable<Text>,
        vl_tot_ded_ger -> Nullable<Text>,
        vl_tot_ded_esp -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i199 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_det -> Nullable<Text>,
        vl_det -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i299 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comp -> Nullable<Text>,
        vl_comp -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i399 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_i990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_i -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        ind_cred_ori -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_cred -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        vl_cred_disp -> Nullable<Text>,
        ind_desc_cred -> Nullable<Text>,
        vl_cred_desc -> Nullable<Text>,
        sld_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis_tot -> Nullable<Text>,
        vl_bc_pis_cum -> Nullable<Text>,
        vl_bc_pis_nc -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        quant_bc_pis_tot -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        desc_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m200 (id) {
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

diesel::table! {
    efd_m205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        vl_debito -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_rec_brt -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
        vl_ajus_acres_bc_pis -> Nullable<Text>,
        vl_ajus_reduc_bc_pis -> Nullable<Text>,
        vl_bc_cont_ajus -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_quant -> Nullable<Text>,
        vl_cont_apur -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cont_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        vl_cont_per -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m211 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_tip_coop -> Nullable<Text>,
        vl_bc_cont_ant_exc_coop -> Nullable<Text>,
        vl_exc_coop_ger -> Nullable<Text>,
        vl_exc_esp_coop -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m230 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_vend -> Nullable<Text>,
        vl_nao_receb -> Nullable<Text>,
        vl_cont_dif -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_cont_apur_difer -> Nullable<Text>,
        nat_cred_desc -> Nullable<Text>,
        vl_cred_desc_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        per_apur -> Nullable<Text>,
        dt_receb -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_fol -> Nullable<Text>,
        vl_exc_bc -> Nullable<Text>,
        vl_tot_bc -> Nullable<Text>,
        aliq_pis_fol -> Nullable<Text>,
        vl_tot_cont_fol -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m410 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_rec -> Nullable<Text>,
        vl_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        ind_cred_ori -> Nullable<Text>,
        vl_bc_cred -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cred -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        vl_cred_disp -> Nullable<Text>,
        ind_desc_cred -> Nullable<Text>,
        vl_cred_desc -> Nullable<Text>,
        sld_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m505 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins_tot -> Nullable<Text>,
        vl_bc_cofins_cum -> Nullable<Text>,
        vl_bc_cofins_nc -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        quant_bc_cofins_tot -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        desc_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}

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

diesel::table! {
    efd_m605 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        vl_debito -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m610 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_rec_brt -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
        vl_ajus_acres_bc_cofins -> Nullable<Text>,
        vl_ajus_reduc_bc_cofins -> Nullable<Text>,
        vl_bc_cont_ajus -> Nullable<Text>,
        aliq_cofins -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_quant -> Nullable<Text>,
        vl_cont_apur -> Nullable<Text>,
        vl_ajus_acres -> Nullable<Text>,
        vl_ajus_reduc -> Nullable<Text>,
        vl_cont_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        vl_cont_per -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m611 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_tip_coop -> Nullable<Text>,
        vl_bc_cont_ant_exc_coop -> Nullable<Text>,
        vl_exc_coop_ger -> Nullable<Text>,
        vl_exc_esp_coop -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m620 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m630 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_vend -> Nullable<Text>,
        vl_nao_receb -> Nullable<Text>,
        vl_cont_dif -> Nullable<Text>,
        vl_cred_dif -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_cont_apur_difer -> Nullable<Text>,
        nat_bc_cred_desc -> Nullable<Text>,
        vl_cred_desc_difer -> Nullable<Text>,
        vl_cont_difer_ant -> Nullable<Text>,
        per_apur -> Nullable<Text>,
        dt_receb -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_tot_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m810 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_rec -> Nullable<Text>,
        vl_rec -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    efd_m990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_m -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fim -> Nullable<Text>,
        vl_rec_tot_est -> Nullable<Text>,
        cod_ativ_econ -> Nullable<Text>,
        vl_rec_ativ_estab -> Nullable<Text>,
        vl_exc -> Nullable<Text>,
        vl_bc_cont -> Nullable<Text>,
        aliq_cont -> Nullable<Text>,
        vl_cont_apu -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

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

diesel::table! {
    efd_p199 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_ref -> Nullable<Text>,
        vl_tot_cont_apu -> Nullable<Text>,
        vl_tot_aj_reduc -> Nullable<Text>,
        vl_tot_aj_acres -> Nullable<Text>,
        vl_tot_cont_dev -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Text>,
    }
}

diesel::table! {
    efd_p990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_p -> Nullable<Text>,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        sped_type -> Text,
    }
}

diesel::table! {
    reg_0000 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        cod_fin -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        nome -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        uf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        im -> Nullable<Text>,
        suframa -> Nullable<Text>,
        ind_perfil -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0002 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        clas_estab_ind -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0005 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        fantasia -> Nullable<Text>,
        cep -> Nullable<Text>,
        endereco -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
        fone -> Nullable<Text>,
        fax -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0015 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf_st -> Nullable<Text>,
        ie_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nome -> Nullable<Text>,
        cpf -> Nullable<Text>,
        crc -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cep -> Nullable<Text>,
        endereco -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
        fone -> Nullable<Text>,
        fax -> Nullable<Text>,
        email -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        nome -> Nullable<Text>,
        cod_pais -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        ie -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        suframa -> Nullable<Text>,
        endereco -> Nullable<Text>,
        num -> Nullable<Text>,
        compl -> Nullable<Text>,
        bairro -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0175 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        nr_campo -> Nullable<Text>,
        cont_ant -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid -> Nullable<Text>,
        descr -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_item -> Nullable<Text>,
        cod_barra -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
        unid_inv -> Nullable<Text>,
        tipo_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        cod_gen -> Nullable<Text>,
        cod_lst -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0205 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        descr_ant_item -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fim -> Nullable<Text>,
        cod_ant_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0206 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comb -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item_comp -> Nullable<Text>,
        qtd_comp -> Nullable<Text>,
        perda -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid_conv -> Nullable<Text>,
        fat_conv -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ind_bem -> Nullable<Text>,
        ident_merc -> Nullable<Text>,
        descr_item -> Nullable<Text>,
        cod_prnc -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        nr_parc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0305 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        func -> Nullable<Text>,
        vida_util -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        descr_nat -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0450 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0460 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_nat_cc -> Nullable<Text>,
        ind_cta -> Nullable<Text>,
        nivel -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        nome_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        ccus -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_0 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_exp -> Nullable<Text>,
        ind_ccrf -> Nullable<Text>,
        ind_comb -> Nullable<Text>,
        ind_usina -> Nullable<Text>,
        ind_va -> Nullable<Text>,
        ind_ee -> Nullable<Text>,
        ind_cart -> Nullable<Text>,
        ind_form -> Nullable<Text>,
        ind_aer -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_doc -> Nullable<Text>,
        nro_de -> Nullable<Text>,
        dt_de -> Nullable<Text>,
        nat_exp -> Nullable<Text>,
        nro_re -> Nullable<Text>,
        dt_re -> Nullable<Text>,
        chc_emb -> Nullable<Text>,
        dt_chc -> Nullable<Text>,
        dt_avb -> Nullable<Text>,
        tp_chc -> Nullable<Text>,
        pais -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1105 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        nr_memo -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        sld_cred -> Nullable<Text>,
        cred_apr -> Nullable<Text>,
        cred_receb -> Nullable<Text>,
        cred_util -> Nullable<Text>,
        sld_cred_fim -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        tipo_util -> Nullable<Text>,
        nr_doc -> Nullable<Text>,
        vl_cred_util -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_fech -> Nullable<Text>,
        estq_abert -> Nullable<Text>,
        vol_entr -> Nullable<Text>,
        vol_disp -> Nullable<Text>,
        vol_saidas -> Nullable<Text>,
        estq_escr -> Nullable<Text>,
        val_aj_perda -> Nullable<Text>,
        val_aj_ganho -> Nullable<Text>,
        fech_fisico -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1310 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        estq_abert -> Nullable<Text>,
        vol_entr -> Nullable<Text>,
        vol_disp -> Nullable<Text>,
        vol_saidas -> Nullable<Text>,
        estq_escr -> Nullable<Text>,
        val_aj_perda -> Nullable<Text>,
        val_aj_ganho -> Nullable<Text>,
        fech_fisico -> Nullable<Text>,
    }
}

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

diesel::table! {
    reg_1350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        serie -> Nullable<Text>,
        fabricante -> Nullable<Text>,
        modelo -> Nullable<Text>,
        tipo_medicao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1360 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_lacre -> Nullable<Text>,
        dat_aplicacao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_bico -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1390 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_prod -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1391 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_registro -> Nullable<Text>,
        qtd_moid -> Nullable<Text>,
        estq_ini -> Nullable<Text>,
        qtd_produz -> Nullable<Text>,
        ent_anid_hid -> Nullable<Text>,
        outr_entr -> Nullable<Text>,
        perda -> Nullable<Text>,
        cons -> Nullable<Text>,
        sai_ani_hid -> Nullable<Text>,
        saidas -> Nullable<Text>,
        estq_fin -> Nullable<Text>,
        estq_ini_mel -> Nullable<Text>,
        prod_dia_mel -> Nullable<Text>,
        util_mel -> Nullable<Text>,
        prod_alc_mel -> Nullable<Text>,
        obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        mun -> Nullable<Text>,
        valor -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_e_s -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_forn -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofis -> Nullable<Text>,
        tp_ligacao -> Nullable<Text>,
        cod_grupo_tensao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        aliq_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        tot_credito -> Nullable<Text>,
        tot_debito -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_disp -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        num_aut -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1710 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_carga -> Nullable<Text>,
        vl_pass -> Nullable<Text>,
        vl_fat -> Nullable<Text>,
        ind_rat -> Nullable<Text>,
        vl_icms_ant -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms_apur -> Nullable<Text>,
        vl_bc_icms_apur -> Nullable<Text>,
        vl_dif -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_apur_icms -> Nullable<Text>,
        descr_compl_out_apur -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1910 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1920 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_transf_debitos_oa -> Nullable<Text>,
        vl_tot_aj_debitos_oa -> Nullable<Text>,
        vl_estornos_cred_oa -> Nullable<Text>,
        vl_tot_transf_creditos_oa -> Nullable<Text>,
        vl_tot_aj_creditos_oa -> Nullable<Text>,
        vl_estornos_deb_oa -> Nullable<Text>,
        vl_sld_credor_ant_oa -> Nullable<Text>,
        vl_sld_apurado_oa -> Nullable<Text>,
        vl_tot_ded -> Nullable<Text>,
        vl_icms_recolher_oa -> Nullable<Text>,
        vl_sld_credor_transp_oa -> Nullable<Text>,
        deb_esp_oa -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1921 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1922 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_da -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1923 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1925 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf_adic -> Nullable<Text>,
        vl_inf_adic -> Nullable<Text>,
        desc_compl_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1926 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        mes_ref -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_1 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9900 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        reg_blc -> Nullable<Text>,
        qtd_reg_blc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_9 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9999 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_e_s -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_abat_nt -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        ind_frt -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        vl_seg -> Nullable<Text>,
        vl_out_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        vl_pis_st -> Nullable<Text>,
        vl_cofins_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c101 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_fcp_uf_dest -> Nullable<Text>,
        vl_icms_uf_dest -> Nullable<Text>,
        vl_icms_uf_rem -> Nullable<Text>,
    }
}

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

diesel::table! {
    reg_c110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

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

diesel::table! {
    reg_c113 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c114 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c115 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_carga -> Nullable<Text>,
        cnpj_col -> Nullable<Text>,
        ie_col -> Nullable<Text>,
        cpf_col -> Nullable<Text>,
        cod_mun_col -> Nullable<Text>,
        cnpj_entg -> Nullable<Text>,
        ie_entg -> Nullable<Text>,
        cpf_entg -> Nullable<Text>,
        cod_mun_entg -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c116 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        num_cfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        pis_imp -> Nullable<Text>,
        cofins_imp -> Nullable<Text>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
        vl_bc_irrf -> Nullable<Text>,
        vl_irrf -> Nullable<Text>,
        vl_bc_prev -> Nullable<Text>,
        vl_prev -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        ind_tit -> Nullable<Text>,
        desc_tit -> Nullable<Text>,
        num_tit -> Nullable<Text>,
        qtd_parc -> Nullable<Text>,
        vl_tit -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c141 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        vl_parc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c160 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c165 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        cod_aut -> Nullable<Text>,
        nr_passe -> Nullable<Text>,
        hora -> Nullable<Text>,
        temper -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
        nom_mot -> Nullable<Text>,
        cpf -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c170 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        aliq_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_apur -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        cod_enq -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Text>,
        aliq_ipi -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis_perc -> Nullable<Text>,
        quant_bc_pis -> Nullable<Text>,
        aliq_pis_reais -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Text>,
        aliq_cofins_perc -> Nullable<Text>,
        quant_bc_cofins -> Nullable<Text>,
        aliq_cofins_reais -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c171 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        qtde -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c172 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        aliq_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c173 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        lote_med -> Nullable<Text>,
        qtd_item -> Nullable<Text>,
        dt_fab -> Nullable<Text>,
        dt_val -> Nullable<Text>,
        ind_med -> Nullable<Text>,
        tp_prod -> Nullable<Text>,
        vl_tab_max -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c174 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_arm -> Nullable<Text>,
        num_arm -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c175 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_veic_oper -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        uf -> Nullable<Text>,
        chassi_veic -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c176 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod_ult_e -> Nullable<Text>,
        num_doc_ult_e -> Nullable<Text>,
        ser_ult_e -> Nullable<Text>,
        dt_ult_e -> Nullable<Text>,
        cod_part_ult_e -> Nullable<Text>,
        quant_ult_e -> Nullable<Text>,
        vl_unit_ult_e -> Nullable<Text>,
        vl_unit_bc_st -> Nullable<Text>,
    }
}

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

diesel::table! {
    reg_c179 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        bc_st_orig_dest -> Nullable<Text>,
        icms_st_rep -> Nullable<Text>,
        icms_st_compl -> Nullable<Text>,
        bc_ret -> Nullable<Text>,
        icms_ret -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c195 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c197 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_outros -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c310 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c320 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c321 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c390 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c405 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c410 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c420 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tot_par -> Nullable<Text>,
        vlr_acum_tot -> Nullable<Text>,
        nr_tot -> Nullable<Text>,
        descr_nr_tot -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c425 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c460 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cpf_cnpj -> Nullable<Text>,
        nome_adq -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c465 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        num_ccf -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c470 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c490 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c495 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_canc -> Nullable<Text>,
        vl_acmo -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_isen -> Nullable<Text>,
        vl_nt -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_e_s -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_forn -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        tp_ligacao -> Nullable<Text>,
        cod_grupo_tensao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        aliq_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c590 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cons -> Nullable<Text>,
        vl_forn -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c601 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c610 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c690 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c700 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        nro_ord_ini -> Nullable<Text>,
        nro_ord_fin -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        nom_mest -> Nullable<Text>,
        chv_cod_dig -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c790 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c791 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c800 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_cfe -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_cfe -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        vl_out_da -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis_st -> Nullable<Text>,
        vl_cofins_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c850 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c860 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        doc_ini -> Nullable<Text>,
        doc_fim -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c890 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_c -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_cte -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_a_p -> Nullable<Text>,
        tp_ct_e -> Nullable<Text>,
        chv_cte_ref -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        ind_frt -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_nt -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d101 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_out -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d120 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        ind_frt_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        vl_liq_frt -> Nullable<Text>,
        vl_sec_cat -> Nullable<Text>,
        vl_desp -> Nullable<Text>,
        vl_pedg -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        ind_veic -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        ind_nav -> Nullable<Text>,
        viagem -> Nullable<Text>,
        vl_frt_liq -> Nullable<Text>,
        vl_desp_port -> Nullable<Text>,
        vl_desp_car_desc -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt_brt -> Nullable<Text>,
        vl_frt_mm -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d150 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        viagem -> Nullable<Text>,
        ind_tfa -> Nullable<Text>,
        vl_peso_tx -> Nullable<Text>,
        vl_tx_terr -> Nullable<Text>,
        vl_tx_red -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_tx_adv -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d160 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        despacho -> Nullable<Text>,
        cnpj_cpf_rem -> Nullable<Text>,
        ie_rem -> Nullable<Text>,
        cod_mun_ori -> Nullable<Text>,
        cnpj_cpf_dest -> Nullable<Text>,
        ie_dest -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d161 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_carga -> Nullable<Text>,
        cnpj_cpf_col -> Nullable<Text>,
        ie_col -> Nullable<Text>,
        cod_mun_col -> Nullable<Text>,
        cnpj_cpf_entg -> Nullable<Text>,
        ie_entg -> Nullable<Text>,
        cod_mun_entg -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d162 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_merc -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Text>,
        peso_liq -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d170 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        otm -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_liq_frt -> Nullable<Text>,
        vl_gris -> Nullable<Text>,
        vl_pdg -> Nullable<Text>,
        vl_out -> Nullable<Text>,
        vl_frt -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d180 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_seq -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cnpj_cpf_emit -> Nullable<Text>,
        uf_emit -> Nullable<Text>,
        ie_emit -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cnpj_cpf_tom -> Nullable<Text>,
        uf_tom -> Nullable<Text>,
        ie_tom -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d190 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d195 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d197 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_outros -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d300 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_seg -> Nullable<Text>,
        vl_out_desp -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d301 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d310 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d350 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d355 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Text>,
        vl_brt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d360 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d365 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tot_par -> Nullable<Text>,
        vlr_acum_tot -> Nullable<Text>,
        nr_tot -> Nullable<Text>,
        descr_nr_tot -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d370 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        qtd_bilh -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d390 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Text>,
        aliq_issqn -> Nullable<Text>,
        vl_issqn -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d400 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d410 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d411 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d420 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        dt_a_p -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d530 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_serv -> Nullable<Text>,
        dt_ini_serv -> Nullable<Text>,
        dt_fin_serv -> Nullable<Text>,
        per_fiscal -> Nullable<Text>,
        cod_area -> Nullable<Text>,
        terminal -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d590 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d600 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        vl_doc -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        vl_serv -> Nullable<Text>,
        vl_serv_nt -> Nullable<Text>,
        vl_terc -> Nullable<Text>,
        vl_da -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d610 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        vl_desc -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        vl_pis -> Nullable<Text>,
        vl_cofins -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d690 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d695 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        nro_ord_ini -> Nullable<Text>,
        nro_ord_fin -> Nullable<Text>,
        dt_doc_ini -> Nullable<Text>,
        dt_doc_fin -> Nullable<Text>,
        nom_mest -> Nullable<Text>,
        chv_cod_dig -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d696 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Text>,
        vl_opr -> Nullable<Text>,
        vl_bc_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
        vl_red_bc -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d697 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Text>,
        vl_icms_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_d -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_debitos -> Nullable<Text>,
        vl_aj_debitos -> Nullable<Text>,
        vl_tot_aj_debitos -> Nullable<Text>,
        vl_estornos_cred -> Nullable<Text>,
        vl_tot_creditos -> Nullable<Text>,
        vl_aj_creditos -> Nullable<Text>,
        vl_tot_aj_creditos -> Nullable<Text>,
        vl_estornos_deb -> Nullable<Text>,
        vl_sld_credor_ant -> Nullable<Text>,
        vl_sld_apurado -> Nullable<Text>,
        vl_tot_ded -> Nullable<Text>,
        vl_icms_recolher -> Nullable<Text>,
        vl_sld_credor_transportar -> Nullable<Text>,
        deb_esp -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e111 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e112 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_da -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e113 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e115 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf_adic -> Nullable<Text>,
        vl_inf_adic -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e116 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        mes_ref -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e210 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov_st -> Nullable<Text>,
        vl_sld_cred_ant_st -> Nullable<Text>,
        vl_devol_st -> Nullable<Text>,
        vl_ressarc_st -> Nullable<Text>,
        vl_out_cred_st -> Nullable<Text>,
        vl_aj_creditos_st -> Nullable<Text>,
        vl_retencao_st -> Nullable<Text>,
        vl_out_deb_st -> Nullable<Text>,
        vl_aj_debitos_st -> Nullable<Text>,
        vl_sld_dev_ant_st -> Nullable<Text>,
        vl_deducoes_st -> Nullable<Text>,
        vl_icms_recol_st -> Nullable<Text>,
        vl_sld_cred_st_transportar -> Nullable<Text>,
        deb_esp_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e230 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_da -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e240 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e250 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Text>,
        dt_vcto -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
        #[sql_name = "proc"]
        proc_ -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        mes_ref -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e500 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_apur -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e510 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        vl_cont_ipi -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Text>,
        vl_ipi -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e520 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_sd_ant_ipi -> Nullable<Text>,
        vl_deb_ipi -> Nullable<Text>,
        vl_cred_ipi -> Nullable<Text>,
        vl_od_ipi -> Nullable<Text>,
        vl_oc_ipi -> Nullable<Text>,
        vl_sc_ipi -> Nullable<Text>,
        vl_sd_ipi -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e530 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        ind_doc -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_e -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g110 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        saldo_in_icms -> Nullable<Text>,
        som_parc -> Nullable<Text>,
        vl_trib_exp -> Nullable<Text>,
        vl_total -> Nullable<Text>,
        ind_per_sai -> Nullable<Text>,
        icms_aprop -> Nullable<Text>,
        som_icms_oc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g125 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ind_bem -> Nullable<Text>,
        dt_mov -> Nullable<Text>,
        tipo_mov -> Nullable<Text>,
        vl_imob_icms_op -> Nullable<Text>,
        vl_imob_icms_st -> Nullable<Text>,
        vl_imob_icms_frt -> Nullable<Text>,
        vl_imob_icms_dif -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g126 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Text>,
        vl_trib_oc -> Nullable<Text>,
        vl_total -> Nullable<Text>,
        ind_per_sai -> Nullable<Text>,
        vl_parc_aprop -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g130 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        serie -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe_cte -> Nullable<Text>,
        dt_doc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g140 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_g -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h005 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_inv -> Nullable<Text>,
        vl_inv -> Nullable<Text>,
        mot_inv -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h010 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        unid -> Nullable<Text>,
        qtd -> Nullable<Text>,
        vl_unit -> Nullable<Text>,
        vl_item -> Nullable<Text>,
        ind_prop -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        vl_item_ir -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h020 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        bl_icms -> Nullable<Text>,
        vl_icms -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_h -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k001 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k100 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k200 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_est -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        ind_est -> Nullable<Text>,
        cod_part -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k220 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_mov -> Nullable<Text>,
        cod_item_ori -> Nullable<Text>,
        cod_item_dest -> Nullable<Text>,
        qtd -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k230 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini_op -> Nullable<Text>,
        dt_fin_op -> Nullable<Text>,
        cod_doc_op -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd_enc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k235 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_saida -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        cod_ins_subst -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k250 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_prod -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k255 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_cons -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Text>,
        cod_ins_subst -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k990 (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_h -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(

);
