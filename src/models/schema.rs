diesel::table! {
    contribuicoes_0000 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        tipo_escrit -> Nullable<Text>,
        ind_sit_esp -> Nullable<Text>,
        num_rec_anterior -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
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
    contribuicoes_0001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0035 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_scp -> Nullable<Text>,
        nome_scp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0100 (id) {
        id -> Nullable<Integer>,
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
    contribuicoes_0110 (id) {
        id -> Nullable<Integer>,
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
    contribuicoes_0111 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        rec_bru_ncum_trib_mi -> Nullable<Double>,
        rec_bru_ncum_nt_mi -> Nullable<Double>,
        rec_bru_ncum_exp -> Nullable<Double>,
        rec_bru_cum -> Nullable<Double>,
        rec_bru_total -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_0120 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        mes_dispensa -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0140 (id) {
        id -> Nullable<Integer>,
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
    contribuicoes_0145 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inc_trib -> Nullable<Text>,
        vl_rec_tot -> Nullable<Double>,
        vl_rec_ativ -> Nullable<Double>,
        vl_rec_demais_ativ -> Nullable<Double>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0150 (id) {
        id -> Nullable<Integer>,
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
    contribuicoes_0190 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        unid -> Nullable<Text>,
        descr -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0200 (id) {
        id -> Nullable<Integer>,
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
        aliq_icms -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_0205 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        descr_ant_item -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fim -> Nullable<Date>,
        cod_ant_item -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0206 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comb -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0208 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_tab -> Nullable<Text>,
        cod_gru -> Nullable<Text>,
        marca_com -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0400 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        descr_nat -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0450 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0500 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Date>,
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
    contribuicoes_0600 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Date>,
        cod_ccus -> Nullable<Text>,
        ccus -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_0990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_0 -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_1001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_1010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        id_sec_jud -> Nullable<Text>,
        id_vara -> Nullable<Text>,
        ind_nat_acao -> Nullable<Text>,
        desc_dec_jud -> Nullable<Text>,
        dt_sent_jud -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_1020 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_nat_acao -> Nullable<Text>,
        dt_dec_adm -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_1100 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_apu_cred -> Nullable<Text>,
        orig_cred -> Nullable<Text>,
        cnpj_suc -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_apu -> Nullable<Double>,
        vl_cred_ext_apu -> Nullable<Double>,
        vl_tot_cred_apu -> Nullable<Double>,
        vl_cred_desc_pa_ant -> Nullable<Double>,
        vl_cred_per_pa_ant -> Nullable<Double>,
        vl_cred_dcomp_pa_ant -> Nullable<Double>,
        sd_cred_disp_efd -> Nullable<Double>,
        vl_cred_desc_efd -> Nullable<Double>,
        vl_cred_per_efd -> Nullable<Double>,
        vl_cred_dcomp_efd -> Nullable<Double>,
        vl_cred_trans -> Nullable<Double>,
        vl_cred_out -> Nullable<Double>,
        sld_cred_fim -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_1300 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        pr_rec_ret -> Nullable<Text>,
        vl_ret_apu -> Nullable<Double>,
        vl_ret_ded -> Nullable<Double>,
        vl_ret_per -> Nullable<Double>,
        vl_ret_dcomp -> Nullable<Double>,
        sld_ret -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_1500 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_apu_cred -> Nullable<Text>,
        orig_cred -> Nullable<Text>,
        cnpj_suc -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_apu -> Nullable<Double>,
        vl_cred_ext_apu -> Nullable<Double>,
        vl_tot_cred_apu -> Nullable<Double>,
        vl_cred_desc_pa_ant -> Nullable<Double>,
        vl_cred_per_pa_ant -> Nullable<Double>,
        vl_cred_dcomp_pa_ant -> Nullable<Double>,
        sd_cred_disp_efd -> Nullable<Double>,
        vl_cred_desc_efd -> Nullable<Double>,
        vl_cred_per_efd -> Nullable<Double>,
        vl_cred_dcomp_efd -> Nullable<Double>,
        vl_cred_trans -> Nullable<Double>,
        vl_cred_out -> Nullable<Double>,
        sld_cred_fim -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_1700 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        pr_rec_ret -> Nullable<Text>,
        vl_ret_apu -> Nullable<Double>,
        vl_ret_ded -> Nullable<Double>,
        vl_ret_per -> Nullable<Double>,
        vl_ret_dcomp -> Nullable<Double>,
        sld_ret -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_1800 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        inc_imob -> Nullable<Text>,
        rec_receb_ret -> Nullable<Double>,
        rec_fin_ret -> Nullable<Double>,
        bc_ret -> Nullable<Double>,
        aliq_ret -> Nullable<Double>,
        vl_rec_uni -> Nullable<Double>,
        dt_rec_uni -> Nullable<Date>,
        cod_rec -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_1900 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        vl_tot_rec -> Nullable<Double>,
        quant_doc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_1990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_1 -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_9001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_9900 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        reg_blc -> Nullable<Text>,
        qtd_reg_blc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_9990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_9 -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_9999 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a100 (id) {
        id -> Nullable<Integer>,
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
        dt_doc -> Nullable<Date>,
        dt_exe_serv -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        vl_pis_ret -> Nullable<Double>,
        vl_cofins_ret -> Nullable<Double>,
        vl_iss -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_a110 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a111 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a120 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_serv -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        vl_pis_imp -> Nullable<Double>,
        dt_pag_pis -> Nullable<Date>,
        vl_bc_cofins -> Nullable<Double>,
        vl_cofins_imp -> Nullable<Double>,
        dt_pag_cofins -> Nullable<Date>,
        loc_exe_serv -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a170 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_a990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_a -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        ind_escri -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c100 (id) {
        id -> Nullable<Integer>,
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
        dt_doc -> Nullable<Date>,
        dt_e_s -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Double>,
        vl_abat_nt -> Nullable<Double>,
        vl_merc -> Nullable<Double>,
        ind_frt -> Nullable<Text>,
        vl_frt -> Nullable<Double>,
        vl_seg -> Nullable<Double>,
        vl_out_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        vl_pis_st -> Nullable<Double>,
        vl_cofins_st -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c110 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c111 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c120 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        vl_pis_imp -> Nullable<Double>,
        vl_cofins_imp -> Nullable<Double>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c170 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        ind_mov -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        aliq_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        ind_apur -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        cod_enq -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Double>,
        aliq_ipi -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c180 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        vl_tot_item -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c181 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c185 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c188 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c190 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_ref_ini -> Nullable<Date>,
        dt_ref_fin -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        cod_ncm -> Nullable<Text>,
        ex_ipi -> Nullable<Text>,
        vl_tot_item -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c191 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj_cpf_part -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c195 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj_cpf_part -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c198 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c199 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        vl_pis_imp -> Nullable<Double>,
        vl_cofins_imp -> Nullable<Double>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c380 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        vl_doc -> Nullable<Double>,
        vl_doc_canc -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c381 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c385 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c395 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c396 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c400 (id) {
        id -> Nullable<Integer>,
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
    contribuicoes_c405 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Double>,
        vl_brt -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c481 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_item -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c485 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_item -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c489 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c490 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        cod_mod -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c491 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c495 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c499 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c500 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        dt_e_s -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c501 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c505 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c509 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c600 (id) {
        id -> Nullable<Integer>,
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
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cons -> Nullable<Text>,
        vl_forn -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_c601 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c605 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c609 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_c990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_c -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d100 (id) {
        id -> Nullable<Integer>,
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
        dt_doc -> Nullable<Date>,
        dt_a_p -> Nullable<Date>,
        tp_cte -> Nullable<Text>,
        chv_cte_ref -> Nullable<Text>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        ind_frt -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_nt -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d101 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d105 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d111 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d200 (id) {
        id -> Nullable<Integer>,
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
        dt_ref -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_d201 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d205 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d209 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d300 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cfop -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d309 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d350 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Double>,
        vl_brt -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d359 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d500 (id) {
        id -> Nullable<Integer>,
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
        dt_doc -> Nullable<Date>,
        dt_a_p -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_d501 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d505 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d509 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d600 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        ind_rec -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_d601 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d605 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d609 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_d990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_d -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f100 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_oper -> Nullable<Date>,
        vl_oper -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        nat_bc_cred -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_doc_oper -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f111 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f120 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ident_bem_imob -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        ind_util_bem_imob -> Nullable<Text>,
        vl_oper_dep -> Nullable<Double>,
        parc_oper_nao_bc_cred -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_bem_imob -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f129 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f130 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        ident_bem_imob -> Nullable<Text>,
        ind_orig_cred -> Nullable<Text>,
        ind_util_bem_imob -> Nullable<Text>,
        mes_oper_aquis -> Nullable<Text>,
        vl_oper_aquis -> Nullable<Double>,
        parc_oper_nao_bc_cred -> Nullable<Double>,
        vl_bc_cred -> Nullable<Double>,
        ind_nr_parc -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        desc_bem_imob -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f139 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f150 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        vl_tot_est -> Nullable<Double>,
        est_imp -> Nullable<Double>,
        vl_bc_est -> Nullable<Double>,
        vl_bc_men_est -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Double>,
        vl_cred_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Double>,
        vl_cred_cofins -> Nullable<Double>,
        desc_est -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f200 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        unid_imob -> Nullable<Text>,
        ident_emp -> Nullable<Text>,
        desc_unid_imob -> Nullable<Text>,
        num_cont -> Nullable<Text>,
        cpf_cnpj_adqu -> Nullable<Text>,
        dt_oper -> Nullable<Date>,
        vl_tot_vend -> Nullable<Double>,
        vl_rec_acum -> Nullable<Double>,
        vl_tot_rec -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        perc_rec_receb -> Nullable<Double>,
        ind_nat_emp -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f205 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_inc_acum_ant -> Nullable<Double>,
        vl_cus_inc_per_esc -> Nullable<Double>,
        vl_cus_inc_acum -> Nullable<Double>,
        vl_exc_bc_cus_inc_acum -> Nullable<Double>,
        vl_bc_cus_inc -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Double>,
        vl_cred_pis_acum -> Nullable<Double>,
        vl_cred_pis_desc_ant -> Nullable<Double>,
        vl_cred_pis_desc -> Nullable<Double>,
        vl_cred_pis_desc_fut -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Double>,
        vl_cred_cofins_acum -> Nullable<Double>,
        vl_cred_cofins_desc_ant -> Nullable<Double>,
        vl_cred_cofins_desc -> Nullable<Double>,
        vl_cred_cofins_desc_fut -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_f210 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_cus_orc -> Nullable<Double>,
        vl_exc -> Nullable<Double>,
        vl_cus_orc_aju -> Nullable<Double>,
        vl_bc_cred -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        aliq_pis -> Nullable<Double>,
        vl_cred_pis_util -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        aliq_cofins -> Nullable<Double>,
        vl_cred_cofins_util -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_f211 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f500 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_caixa -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f509 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f510 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_caixa -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f519 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f525 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec -> Nullable<Double>,
        ind_rec -> Nullable<Text>,
        cnpj_cpf -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_rec_det -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        info_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f550 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_comp -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f559 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f560 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_comp -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_desc_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_desc_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_mod -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f569 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f600 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_ret -> Nullable<Text>,
        dt_ret -> Nullable<Date>,
        vl_bc_ret -> Nullable<Double>,
        vl_ret -> Nullable<Double>,
        cod_rec -> Nullable<Text>,
        ind_nat_rec -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_ret_pis -> Nullable<Double>,
        vl_ret_cofins -> Nullable<Double>,
        ind_dec -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f700 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_ori_ded -> Nullable<Text>,
        ind_nat_ded -> Nullable<Text>,
        vl_ded_pis -> Nullable<Double>,
        vl_ded_cofins -> Nullable<Double>,
        vl_bc_oper -> Nullable<Double>,
        cnpj -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_f800 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_nat_even -> Nullable<Text>,
        dt_even -> Nullable<Date>,
        cnpj_suced -> Nullable<Text>,
        pa_cont_cred -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        vl_cred_pis -> Nullable<Double>,
        vl_cred_cofins -> Nullable<Double>,
        per_cred_cis -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_f990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_f -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        ind_ativ -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i100 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_rec_fin -> Nullable<Double>,
        cst -> Nullable<Text>,
        vl_tot_ded_ger -> Nullable<Double>,
        vl_tot_ded_esp -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i199 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i200 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_det -> Nullable<Text>,
        vl_det -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i299 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i300 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_comp -> Nullable<Text>,
        vl_comp -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        inf_comp -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i399 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_i990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_i -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m100 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        ind_cred_ori -> Nullable<Text>,
        vl_bc_cred -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_cred -> Nullable<Double>,
        vl_ajus_acres -> Nullable<Double>,
        vl_ajus_reduc -> Nullable<Double>,
        vl_cred_dif -> Nullable<Double>,
        vl_cred_disp -> Nullable<Double>,
        ind_desc_cred -> Nullable<Text>,
        vl_cred_desc -> Nullable<Double>,
        sld_cred -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m105 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis_tot -> Nullable<Double>,
        vl_bc_pis_cum -> Nullable<Double>,
        vl_bc_pis_nc -> Nullable<Double>,
        vl_bc_pis -> Nullable<Double>,
        quant_bc_pis_tot -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        desc_cred -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m110 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m200 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_cont_nc_per -> Nullable<Double>,
        vl_tot_cred_desc -> Nullable<Double>,
        vl_tot_cred_desc_ant -> Nullable<Double>,
        vl_tot_cont_nc_dev -> Nullable<Double>,
        vl_ret_nc -> Nullable<Double>,
        vl_out_ded_nc -> Nullable<Double>,
        vl_cont_nc_rec -> Nullable<Double>,
        vl_tot_cont_cum_per -> Nullable<Double>,
        vl_ret_cum -> Nullable<Double>,
        vl_out_ded_cum -> Nullable<Double>,
        vl_cont_cum_rec -> Nullable<Double>,
        vl_tot_cont_rec -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m205 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        vl_debito -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m210 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_rec_brt -> Nullable<Double>,
        vl_bc_cont -> Nullable<Double>,
        aliq_pis -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_quant -> Nullable<Double>,
        vl_cont_apur -> Nullable<Double>,
        vl_ajus_acres -> Nullable<Double>,
        vl_ajus_reduc -> Nullable<Double>,
        vl_cont_difer -> Nullable<Double>,
        vl_cont_difer_ant -> Nullable<Double>,
        vl_cont_per -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m211 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_tip_coop -> Nullable<Text>,
        vl_bc_cont_ant_exc_coop -> Nullable<Double>,
        vl_exc_coop_ger -> Nullable<Double>,
        vl_exc_esp_coop -> Nullable<Double>,
        vl_bc_cont -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m220 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m230 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_vend -> Nullable<Double>,
        vl_nao_receb -> Nullable<Double>,
        vl_cont_dif -> Nullable<Double>,
        vl_cred_dif -> Nullable<Double>,
        cod_cred -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m300 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_cont_apur_difer -> Nullable<Double>,
        nat_cred_desc -> Nullable<Text>,
        vl_cred_desc_difer -> Nullable<Double>,
        vl_cont_difer_ant -> Nullable<Double>,
        per_apur -> Nullable<Text>,
        dt_receb -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m350 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_fol -> Nullable<Double>,
        vl_exc_bc -> Nullable<Double>,
        vl_tot_bc -> Nullable<Double>,
        aliq_pis_fol -> Nullable<Double>,
        vl_tot_cont_fol -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m400 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_pis -> Nullable<Text>,
        vl_tot_rec -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m410 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_rec -> Nullable<Text>,
        vl_rec -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m500 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cred -> Nullable<Text>,
        ind_cred_ori -> Nullable<Text>,
        vl_bc_cred -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cred -> Nullable<Double>,
        vl_ajus_acres -> Nullable<Double>,
        vl_ajus_reduc -> Nullable<Double>,
        vl_cred_dif -> Nullable<Double>,
        vl_cred_disp -> Nullable<Double>,
        ind_desc_cred -> Nullable<Text>,
        vl_cred_desc -> Nullable<Double>,
        sld_cred -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m505 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_bc_cred -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins_tot -> Nullable<Double>,
        vl_bc_cofins_cum -> Nullable<Double>,
        vl_bc_cofins_nc -> Nullable<Double>,
        vl_bc_cofins -> Nullable<Double>,
        quant_bc_cofins_tot -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        desc_cred -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m510 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m600 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        vl_tot_cont_nc_per -> Nullable<Double>,
        vl_tot_cred_desc -> Nullable<Double>,
        vl_tot_cred_desc_ant -> Nullable<Double>,
        vl_tot_cont_nc_dev -> Nullable<Double>,
        vl_ret_nc -> Nullable<Double>,
        vl_out_ded_nc -> Nullable<Double>,
        vl_cont_nc_rec -> Nullable<Double>,
        vl_tot_cont_cum_per -> Nullable<Double>,
        vl_ret_cum -> Nullable<Double>,
        vl_out_ded_cum -> Nullable<Double>,
        vl_cont_cum_rec -> Nullable<Double>,
        vl_tot_cont_rec -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m605 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_rec -> Nullable<Text>,
        vl_debito -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m610 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_rec_brt -> Nullable<Double>,
        vl_bc_cont -> Nullable<Double>,
        aliq_cofins -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_quant -> Nullable<Double>,
        vl_cont_apur -> Nullable<Double>,
        vl_ajus_acres -> Nullable<Double>,
        vl_ajus_reduc -> Nullable<Double>,
        vl_cont_difer -> Nullable<Double>,
        vl_cont_difer_ant -> Nullable<Double>,
        vl_cont_per -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m611 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_tip_coop -> Nullable<Text>,
        vl_bc_cont_ant_exc_coop -> Nullable<Double>,
        vl_exc_coop_ger -> Nullable<Double>,
        vl_exc_esp_coop -> Nullable<Double>,
        vl_bc_cont -> Nullable<Double>,
    }
}

diesel::table! {
    contribuicoes_m620 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m630 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        vl_vend -> Nullable<Double>,
        vl_nao_receb -> Nullable<Double>,
        vl_cont_dif -> Nullable<Double>,
        vl_cred_dif -> Nullable<Double>,
        cod_cred -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m700 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cod_cont -> Nullable<Text>,
        vl_cont_apur_difer -> Nullable<Double>,
        nat_bc_cred_desc -> Nullable<Text>,
        vl_cred_desc_difer -> Nullable<Double>,
        vl_cont_difer_ant -> Nullable<Double>,
        per_apur -> Nullable<Text>,
        dt_receb -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_m800 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cst_cofins -> Nullable<Text>,
        vl_tot_rec -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m810 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        nat_rec -> Nullable<Text>,
        vl_rec -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        desc_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_m990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_m -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p001 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p010 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        cnpj -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p100 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fim -> Nullable<Date>,
        vl_rec_tot_est -> Nullable<Double>,
        cod_ativ_econ -> Nullable<Text>,
        vl_rec_ativ_estab -> Nullable<Double>,
        vl_exc -> Nullable<Double>,
        vl_bc_cont -> Nullable<Double>,
        aliq_cont -> Nullable<Double>,
        vl_cont_apu -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        info_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p110 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_campo -> Nullable<Text>,
        cod_det -> Nullable<Text>,
        det_valor -> Nullable<Double>,
        inf_compl -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p199 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p200 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        per_ref -> Nullable<Text>,
        vl_tot_cont_apu -> Nullable<Double>,
        vl_tot_aj_reduc -> Nullable<Double>,
        vl_tot_aj_acres -> Nullable<Double>,
        vl_tot_cont_dev -> Nullable<Double>,
        cod_rec -> Nullable<Text>,
    }
}

diesel::table! {
    contribuicoes_p210 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
        dt_ref -> Nullable<Date>,
    }
}

diesel::table! {
    contribuicoes_p990 (id) {
        id -> Nullable<Integer>,
        file_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        reg -> Nullable<Text>,
        qtd_lin_p -> Nullable<Text>,
    }
}

diesel::table! {
    files (ID) {
        ID -> Nullable<Integer>,
        NAME -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0000 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_ver -> Nullable<Text>,
        cod_fin -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
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
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0005 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        uf_st -> Nullable<Text>,
        ie_st -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0100 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Date>,
        nr_campo -> Nullable<Text>,
        cont_ant -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0190 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        unid -> Nullable<Text>,
        descr -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0200 (id) {
        id -> Integer,
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
        aliq_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_0205 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        descr_ant_item -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fim -> Nullable<Date>,
        cod_ant_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0206 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_comb -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0210 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item_comp -> Nullable<Text>,
        qtd_comp -> Nullable<Double>,
        perda -> Nullable<Double>,
    }
}

diesel::table! {
    reg_0220 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        unid_conv -> Nullable<Text>,
        fat_conv -> Nullable<Double>,
    }
}

diesel::table! {
    reg_0300 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_ccus -> Nullable<Text>,
        func -> Nullable<Text>,
        vida_util -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0400 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        descr_nat -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0450 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0460 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0500 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Date>,
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
        reg -> Nullable<Text>,
        dt_alt -> Nullable<Date>,
        cod_ccus -> Nullable<Text>,
        ccus -> Nullable<Text>,
    }
}

diesel::table! {
    reg_0990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_0 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1010 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        ind_doc -> Nullable<Text>,
        nro_de -> Nullable<Text>,
        dt_de -> Nullable<Date>,
        nat_exp -> Nullable<Text>,
        nro_re -> Nullable<Text>,
        dt_re -> Nullable<Date>,
        chc_emb -> Nullable<Text>,
        dt_chc -> Nullable<Date>,
        dt_avb -> Nullable<Date>,
        tp_chc -> Nullable<Text>,
        pais -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1105 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cod_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1110 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        chv_nfe -> Nullable<Text>,
        nr_memo -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1200 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        sld_cred -> Nullable<Double>,
        cred_apr -> Nullable<Double>,
        cred_receb -> Nullable<Double>,
        cred_util -> Nullable<Double>,
        sld_cred_fim -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1210 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        tipo_util -> Nullable<Text>,
        nr_doc -> Nullable<Text>,
        vl_cred_util -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1300 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        dt_fech -> Nullable<Date>,
        estq_abert -> Nullable<Double>,
        vol_entr -> Nullable<Double>,
        vol_disp -> Nullable<Double>,
        vol_saidas -> Nullable<Double>,
        estq_escr -> Nullable<Double>,
        val_aj_perda -> Nullable<Double>,
        val_aj_ganho -> Nullable<Double>,
        fech_fisico -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1310 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        estq_abert -> Nullable<Double>,
        vol_entr -> Nullable<Double>,
        vol_disp -> Nullable<Double>,
        vol_saidas -> Nullable<Double>,
        estq_escr -> Nullable<Double>,
        val_aj_perda -> Nullable<Double>,
        val_aj_ganho -> Nullable<Double>,
        fech_fisico -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1320 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_bico -> Nullable<Text>,
        nr_interv -> Nullable<Text>,
        mot_interv -> Nullable<Text>,
        nom_interv -> Nullable<Text>,
        cnpj_interv -> Nullable<Text>,
        cpf_interv -> Nullable<Text>,
        val_fecha -> Nullable<Double>,
        val_abert -> Nullable<Double>,
        vol_aferi -> Nullable<Double>,
        vol_vendas -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1350 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        num_lacre -> Nullable<Text>,
        dat_aplicacao -> Nullable<Date>,
    }
}

diesel::table! {
    reg_1370 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_bico -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1390 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_prod -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1391 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_registro -> Nullable<Date>,
        qtd_moid -> Nullable<Double>,
        estq_ini -> Nullable<Double>,
        qtd_produz -> Nullable<Double>,
        ent_anid_hid -> Nullable<Double>,
        outr_entr -> Nullable<Double>,
        perda -> Nullable<Double>,
        cons -> Nullable<Double>,
        sai_ani_hid -> Nullable<Double>,
        saidas -> Nullable<Double>,
        estq_fin -> Nullable<Double>,
        estq_ini_mel -> Nullable<Double>,
        prod_dia_mel -> Nullable<Double>,
        util_mel -> Nullable<Double>,
        prod_alc_mel -> Nullable<Double>,
        obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1400 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        mun -> Nullable<Text>,
        valor -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1500 (id) {
        id -> Integer,
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
        dt_doc -> Nullable<Date>,
        dt_e_s -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_forn -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofis -> Nullable<Double>,
        tp_ligacao -> Nullable<Text>,
        cod_grupo_tensao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1510 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        aliq_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1600 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        tot_credito -> Nullable<Double>,
        tot_debito -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1700 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1800 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_carga -> Nullable<Double>,
        vl_pass -> Nullable<Double>,
        vl_fat -> Nullable<Double>,
        ind_rat -> Nullable<Double>,
        vl_icms_ant -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms_apur -> Nullable<Double>,
        vl_bc_icms_apur -> Nullable<Double>,
        vl_dif -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1900 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_apur_icms -> Nullable<Text>,
        descr_compl_out_apur -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1910 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
    }
}

diesel::table! {
    reg_1920 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_tot_transf_debitos_oa -> Nullable<Double>,
        vl_tot_aj_debitos_oa -> Nullable<Double>,
        vl_estornos_cred_oa -> Nullable<Double>,
        vl_tot_transf_creditos_oa -> Nullable<Double>,
        vl_tot_aj_creditos_oa -> Nullable<Double>,
        vl_estornos_deb_oa -> Nullable<Double>,
        vl_sld_credor_ant_oa -> Nullable<Double>,
        vl_sld_apurado_oa -> Nullable<Double>,
        vl_tot_ded -> Nullable<Double>,
        vl_icms_recolher_oa -> Nullable<Double>,
        vl_sld_credor_transp_oa -> Nullable<Double>,
        deb_esp_oa -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1921 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1922 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Double>,
    }
}

diesel::table! {
    reg_1925 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_inf_adic -> Nullable<Text>,
        vl_inf_adic -> Nullable<Double>,
        desc_compl_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_1926 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Double>,
        dt_vcto -> Nullable<Date>,
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
        reg -> Nullable<Text>,
        qtd_lin_1 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9900 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        reg_blc -> Nullable<Text>,
        qtd_reg_blc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_9 -> Nullable<Text>,
    }
}

diesel::table! {
    reg_9999 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c100 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        dt_e_s -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        ind_pgto -> Nullable<Text>,
        vl_desc -> Nullable<Double>,
        vl_abat_nt -> Nullable<Double>,
        vl_merc -> Nullable<Double>,
        ind_frt -> Nullable<Text>,
        vl_frt -> Nullable<Double>,
        vl_seg -> Nullable<Double>,
        vl_out_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        vl_pis_st -> Nullable<Double>,
        vl_cofins_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c105 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        oper -> Nullable<Text>,
        cod_uf -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c110 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_inf -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c111 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_proc -> Nullable<Text>,
        ind_proc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c112 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_da -> Nullable<Text>,
        uf -> Nullable<Text>,
        num_da -> Nullable<Text>,
        cod_aut -> Nullable<Text>,
        vl_da -> Nullable<Double>,
        dt_vcto -> Nullable<Date>,
        dt_pgto -> Nullable<Date>,
    }
}

diesel::table! {
    reg_c113 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
    }
}

diesel::table! {
    reg_c114 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ecf_fab -> Nullable<Text>,
        ecf_cx -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
    }
}

diesel::table! {
    reg_c115 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_doc_imp -> Nullable<Text>,
        num_doc_imp -> Nullable<Text>,
        pis_imp -> Nullable<Double>,
        cofins_imp -> Nullable<Double>,
        num_acdraw -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c130 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_serv_nt -> Nullable<Double>,
        vl_bc_issqn -> Nullable<Double>,
        vl_issqn -> Nullable<Double>,
        vl_bc_irrf -> Nullable<Double>,
        vl_irrf -> Nullable<Double>,
        vl_bc_prev -> Nullable<Double>,
        vl_prev -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c140 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        ind_tit -> Nullable<Text>,
        desc_tit -> Nullable<Text>,
        num_tit -> Nullable<Text>,
        qtd_parc -> Nullable<Text>,
        vl_tit -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c141 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_parc -> Nullable<Text>,
        dt_vcto -> Nullable<Date>,
        vl_parc -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c160 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Double>,
        peso_liq -> Nullable<Double>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c165 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        cod_aut -> Nullable<Text>,
        nr_passe -> Nullable<Text>,
        hora -> Nullable<Text>,
        temper -> Nullable<Double>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Double>,
        peso_liq -> Nullable<Double>,
        nom_mot -> Nullable<Text>,
        cpf -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c170 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        ind_mov -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cod_nat -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        aliq_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        ind_apur -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        cod_enq -> Nullable<Text>,
        vl_bc_ipi -> Nullable<Double>,
        aliq_ipi -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
        cst_pis -> Nullable<Text>,
        vl_bc_pis -> Nullable<Double>,
        aliq_pis_perc -> Nullable<Double>,
        quant_bc_pis -> Nullable<Double>,
        aliq_pis_reais -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        cst_cofins -> Nullable<Text>,
        vl_bc_cofins -> Nullable<Double>,
        aliq_cofins_perc -> Nullable<Double>,
        quant_bc_cofins -> Nullable<Double>,
        aliq_cofins_reais -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c171 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_tanque -> Nullable<Text>,
        qtde -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c172 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_bc_issqn -> Nullable<Double>,
        aliq_issqn -> Nullable<Double>,
        vl_issqn -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c173 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        lote_med -> Nullable<Text>,
        qtd_item -> Nullable<Double>,
        dt_fab -> Nullable<Date>,
        dt_val -> Nullable<Date>,
        ind_med -> Nullable<Text>,
        tp_prod -> Nullable<Text>,
        vl_tab_max -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c174 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_arm -> Nullable<Text>,
        num_arm -> Nullable<Text>,
        descr_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c175 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_mod_ult_e -> Nullable<Text>,
        num_doc_ult_e -> Nullable<Text>,
        ser_ult_e -> Nullable<Text>,
        dt_ult_e -> Nullable<Date>,
        cod_part_ult_e -> Nullable<Text>,
        quant_ult_e -> Nullable<Double>,
        vl_unit_ult_e -> Nullable<Double>,
        vl_unit_bc_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c177 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_selo_ipi -> Nullable<Text>,
        qt_selo_ipi -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c178 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cl_enq -> Nullable<Text>,
        vl_unid -> Nullable<Double>,
        quant_pad -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c179 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        bc_st_orig_dest -> Nullable<Double>,
        icms_st_rep -> Nullable<Double>,
        icms_st_compl -> Nullable<Double>,
        bc_ret -> Nullable<Double>,
        icms_ret -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c190 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c195 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c197 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_outros -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c300 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c310 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c320 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c321 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c350 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub_ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cnpj_cpf -> Nullable<Text>,
        vl_merc -> Nullable<Double>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofis -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c370 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c390 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c400 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Double>,
        vl_brt -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c410 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c420 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_tot_par -> Nullable<Text>,
        vlr_acum_tot -> Nullable<Double>,
        nr_tot -> Nullable<Text>,
        descr_nr_tot -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c425 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c460 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cpf_cnpj -> Nullable<Text>,
        nome_adq -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c465 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        num_ccf -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c470 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        qtd_canc -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c490 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c495 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        qtd_canc -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_canc -> Nullable<Double>,
        vl_acmo -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_isen -> Nullable<Double>,
        vl_nt -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c500 (id) {
        id -> Integer,
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
        dt_doc -> Nullable<Date>,
        dt_e_s -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_forn -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        tp_ligacao -> Nullable<Text>,
        cod_grupo_tensao -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c510 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        aliq_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c590 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c600 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        qtd_canc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cons -> Nullable<Text>,
        vl_forn -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c601 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c610 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c690 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c700 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        nro_ord_ini -> Nullable<Text>,
        nro_ord_fin -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        nom_mest -> Nullable<Text>,
        chv_cod_dig -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c790 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c791 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c800 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        num_cfe -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_cfe -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cnpj_cpf -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        chv_cfe -> Nullable<Text>,
        vl_desc -> Nullable<Double>,
        vl_merc -> Nullable<Double>,
        vl_out_da -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_pis_st -> Nullable<Double>,
        vl_cofins_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_c850 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c860 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        nr_sat -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        doc_ini -> Nullable<Text>,
        doc_fim -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c890 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_c990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_c -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d100 (id) {
        id -> Integer,
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
        dt_doc -> Nullable<Date>,
        dt_a_p -> Nullable<Date>,
        tp_ct_e -> Nullable<Text>,
        chv_cte_ref -> Nullable<Text>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        ind_frt -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_nt -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d110 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        vl_out -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d120 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        ind_frt_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        vl_liq_frt -> Nullable<Double>,
        vl_sec_cat -> Nullable<Double>,
        vl_desp -> Nullable<Double>,
        vl_pedg -> Nullable<Double>,
        vl_out -> Nullable<Double>,
        vl_frt -> Nullable<Double>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d140 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        ind_veic -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        ind_nav -> Nullable<Text>,
        viagem -> Nullable<Text>,
        vl_frt_liq -> Nullable<Double>,
        vl_desp_port -> Nullable<Double>,
        vl_desp_car_desc -> Nullable<Double>,
        vl_out -> Nullable<Double>,
        vl_frt_brt -> Nullable<Double>,
        vl_frt_mm -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d150 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        veic_id -> Nullable<Text>,
        viagem -> Nullable<Text>,
        ind_tfa -> Nullable<Text>,
        vl_peso_tx -> Nullable<Double>,
        vl_tx_terr -> Nullable<Double>,
        vl_tx_red -> Nullable<Double>,
        vl_out -> Nullable<Double>,
        vl_tx_adv -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d160 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_merc -> Nullable<Double>,
        qtd_vol -> Nullable<Text>,
        peso_brt -> Nullable<Double>,
        peso_liq -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d170 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part_consg -> Nullable<Text>,
        cod_part_red -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        cod_mun_dest -> Nullable<Text>,
        otm -> Nullable<Text>,
        ind_nat_frt -> Nullable<Text>,
        vl_liq_frt -> Nullable<Double>,
        vl_gris -> Nullable<Double>,
        vl_pdg -> Nullable<Double>,
        vl_out -> Nullable<Double>,
        vl_frt -> Nullable<Double>,
        veic_id -> Nullable<Text>,
        uf_id -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d180 (id) {
        id -> Integer,
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
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d190 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d195 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_obs -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d197 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_outros -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d300 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        dt_doc -> Nullable<Date>,
        vl_opr -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_seg -> Nullable<Double>,
        vl_out_desp -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d301 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d310 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d350 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cro -> Nullable<Text>,
        crz -> Nullable<Text>,
        num_coo_fin -> Nullable<Text>,
        gt_fin -> Nullable<Double>,
        vl_brt -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d360 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d365 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_tot_par -> Nullable<Text>,
        vlr_acum_tot -> Nullable<Double>,
        nr_tot -> Nullable<Text>,
        descr_nr_tot -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d370 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        qtd_bilh -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d390 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_issqn -> Nullable<Double>,
        aliq_issqn -> Nullable<Double>,
        vl_issqn -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d400 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d410 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc_ini -> Nullable<Text>,
        num_doc_fin -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d411 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_doc_canc -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d420 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mun_orig -> Nullable<Text>,
        vl_serv -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d500 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_oper -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_sit -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        dt_a_p -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        cod_inf -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
        tp_assinante -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d510 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        vl_bc_icms -> Nullable<Double>,
        aliq_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        ind_rec -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d530 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_serv -> Nullable<Text>,
        dt_ini_serv -> Nullable<Date>,
        dt_fin_serv -> Nullable<Date>,
        per_fiscal -> Nullable<Text>,
        cod_area -> Nullable<Text>,
        terminal -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d590 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d600 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        cod_mun -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        cod_cons -> Nullable<Text>,
        qtd_cons -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        vl_doc -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        vl_serv -> Nullable<Double>,
        vl_serv_nt -> Nullable<Double>,
        vl_terc -> Nullable<Double>,
        vl_da -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d610 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_class -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        unid -> Nullable<Text>,
        vl_item -> Nullable<Double>,
        vl_desc -> Nullable<Double>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        vl_pis -> Nullable<Double>,
        vl_cofins -> Nullable<Double>,
        cod_cta -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d690 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d695 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        nro_ord_ini -> Nullable<Text>,
        nro_ord_fin -> Nullable<Text>,
        dt_doc_ini -> Nullable<Date>,
        dt_doc_fin -> Nullable<Date>,
        nom_mest -> Nullable<Text>,
        chv_cod_dig -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d696 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        cfop -> Nullable<Text>,
        aliq_icms -> Nullable<Double>,
        vl_opr -> Nullable<Double>,
        vl_bc_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
        vl_red_bc -> Nullable<Double>,
        cod_obs -> Nullable<Text>,
    }
}

diesel::table! {
    reg_d697 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        vl_bc_icms_st -> Nullable<Double>,
        vl_icms_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_d990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_d -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e100 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
    }
}

diesel::table! {
    reg_e110 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_tot_debitos -> Nullable<Double>,
        vl_aj_debitos -> Nullable<Double>,
        vl_tot_aj_debitos -> Nullable<Double>,
        vl_estornos_cred -> Nullable<Double>,
        vl_tot_creditos -> Nullable<Double>,
        vl_aj_creditos -> Nullable<Double>,
        vl_tot_aj_creditos -> Nullable<Double>,
        vl_estornos_deb -> Nullable<Double>,
        vl_sld_credor_ant -> Nullable<Double>,
        vl_sld_apurado -> Nullable<Double>,
        vl_tot_ded -> Nullable<Double>,
        vl_icms_recolher -> Nullable<Double>,
        vl_sld_credor_transportar -> Nullable<Double>,
        deb_esp -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e111 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e112 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e115 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_inf_adic -> Nullable<Text>,
        vl_inf_adic -> Nullable<Double>,
        descr_compl_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e116 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Double>,
        dt_vcto -> Nullable<Date>,
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
        reg -> Nullable<Text>,
        uf -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
    }
}

diesel::table! {
    reg_e210 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov_st -> Nullable<Text>,
        vl_sld_cred_ant_st -> Nullable<Double>,
        vl_devol_st -> Nullable<Double>,
        vl_ressarc_st -> Nullable<Double>,
        vl_out_cred_st -> Nullable<Double>,
        vl_aj_creditos_st -> Nullable<Double>,
        vl_retencao_st -> Nullable<Double>,
        vl_out_deb_st -> Nullable<Double>,
        vl_aj_debitos_st -> Nullable<Double>,
        vl_sld_dev_ant_st -> Nullable<Double>,
        vl_deducoes_st -> Nullable<Double>,
        vl_icms_recol_st -> Nullable<Double>,
        vl_sld_cred_st_transportar -> Nullable<Double>,
        deb_esp_st -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e220 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_aj_apur -> Nullable<Text>,
        descr_compl_aj -> Nullable<Text>,
        vl_aj_apur -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e230 (id) {
        id -> Integer,
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
        reg -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        ser -> Nullable<Text>,
        sub -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        vl_aj_item -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e250 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_or -> Nullable<Text>,
        vl_or -> Nullable<Double>,
        dt_vcto -> Nullable<Date>,
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
        reg -> Nullable<Text>,
        ind_apur -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
    }
}

diesel::table! {
    reg_e510 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cfop -> Nullable<Text>,
        cst_ipi -> Nullable<Text>,
        vl_cont_ipi -> Nullable<Double>,
        vl_bc_ipi -> Nullable<Double>,
        vl_ipi -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e520 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        vl_sd_ant_ipi -> Nullable<Double>,
        vl_deb_ipi -> Nullable<Double>,
        vl_cred_ipi -> Nullable<Double>,
        vl_od_ipi -> Nullable<Double>,
        vl_oc_ipi -> Nullable<Double>,
        vl_sc_ipi -> Nullable<Double>,
        vl_sd_ipi -> Nullable<Double>,
    }
}

diesel::table! {
    reg_e530 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_aj -> Nullable<Text>,
        vl_aj -> Nullable<Double>,
        cod_aj -> Nullable<Text>,
        ind_doc -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        descr_aj -> Nullable<Text>,
    }
}

diesel::table! {
    reg_e990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_e -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g110 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
        saldo_in_icms -> Nullable<Double>,
        som_parc -> Nullable<Double>,
        vl_trib_exp -> Nullable<Double>,
        vl_total -> Nullable<Double>,
        ind_per_sai -> Nullable<Double>,
        icms_aprop -> Nullable<Double>,
        som_icms_oc -> Nullable<Double>,
    }
}

diesel::table! {
    reg_g125 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_ind_bem -> Nullable<Text>,
        dt_mov -> Nullable<Date>,
        tipo_mov -> Nullable<Text>,
        vl_imob_icms_op -> Nullable<Double>,
        vl_imob_icms_st -> Nullable<Double>,
        vl_imob_icms_frt -> Nullable<Double>,
        vl_imob_icms_dif -> Nullable<Double>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Double>,
    }
}

diesel::table! {
    reg_g126 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_ini -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
        num_parc -> Nullable<Text>,
        vl_parc_pass -> Nullable<Double>,
        vl_trib_oc -> Nullable<Double>,
        vl_total -> Nullable<Double>,
        ind_per_sai -> Nullable<Double>,
        vl_parc_aprop -> Nullable<Double>,
    }
}

diesel::table! {
    reg_g130 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_emit -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        cod_mod -> Nullable<Text>,
        serie -> Nullable<Text>,
        num_doc -> Nullable<Text>,
        chv_nfe_cte -> Nullable<Text>,
        dt_doc -> Nullable<Date>,
    }
}

diesel::table! {
    reg_g140 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        num_item -> Nullable<Text>,
        cod_item -> Nullable<Text>,
    }
}

diesel::table! {
    reg_g990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_g -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h005 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_inv -> Nullable<Date>,
        vl_inv -> Nullable<Double>,
        mot_inv -> Nullable<Text>,
    }
}

diesel::table! {
    reg_h010 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        unid -> Nullable<Text>,
        qtd -> Nullable<Double>,
        vl_unit -> Nullable<Double>,
        vl_item -> Nullable<Double>,
        ind_prop -> Nullable<Text>,
        cod_part -> Nullable<Text>,
        txt_compl -> Nullable<Text>,
        cod_cta -> Nullable<Text>,
        vl_item_ir -> Nullable<Double>,
    }
}

diesel::table! {
    reg_h020 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cst_icms -> Nullable<Text>,
        bl_icms -> Nullable<Double>,
        vl_icms -> Nullable<Double>,
    }
}

diesel::table! {
    reg_h990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_h -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k001 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        ind_mov -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k100 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        cod_item -> Nullable<Date>,
        dt_fin -> Nullable<Date>,
    }
}

diesel::table! {
    reg_k200 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_est -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        ind_est -> Nullable<Text>,
        cod_part -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k220 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_mov -> Nullable<Date>,
        cod_item_ori -> Nullable<Text>,
        cod_item_dest -> Nullable<Text>,
        qtd -> Nullable<Double>,
    }
}

diesel::table! {
    reg_k230 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_ini_op -> Nullable<Date>,
        dt_fin_op -> Nullable<Date>,
        cod_doc_op -> Nullable<Text>,
        cod_item -> Nullable<Text>,
        qtd_enc -> Nullable<Double>,
    }
}

diesel::table! {
    reg_k235 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_saida -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        cod_ins_subst -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k250 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_prod -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
    }
}

diesel::table! {
    reg_k255 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        dt_cons -> Nullable<Date>,
        cod_item -> Nullable<Text>,
        qtd -> Nullable<Double>,
        cod_ins_subst -> Nullable<Text>,
    }
}

diesel::table! {
    reg_k990 (id) {
        id -> Integer,
        reg -> Nullable<Text>,
        qtd_lin_h -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contribuicoes_0000,
    contribuicoes_0001,
    contribuicoes_0035,
    contribuicoes_0100,
    contribuicoes_0110,
    contribuicoes_0111,
    contribuicoes_0120,
    contribuicoes_0140,
    contribuicoes_0145,
    contribuicoes_0150,
    contribuicoes_0190,
    contribuicoes_0200,
    contribuicoes_0205,
    contribuicoes_0206,
    contribuicoes_0208,
    contribuicoes_0400,
    contribuicoes_0450,
    contribuicoes_0500,
    contribuicoes_0600,
    contribuicoes_0990,
    contribuicoes_1001,
    contribuicoes_1010,
    contribuicoes_1020,
    contribuicoes_1100,
    contribuicoes_1300,
    contribuicoes_1500,
    contribuicoes_1700,
    contribuicoes_1800,
    contribuicoes_1900,
    contribuicoes_1990,
    contribuicoes_9001,
    contribuicoes_9900,
    contribuicoes_9990,
    contribuicoes_9999,
    contribuicoes_a001,
    contribuicoes_a010,
    contribuicoes_a100,
    contribuicoes_a110,
    contribuicoes_a111,
    contribuicoes_a120,
    contribuicoes_a170,
    contribuicoes_a990,
    contribuicoes_c001,
    contribuicoes_c010,
    contribuicoes_c100,
    contribuicoes_c110,
    contribuicoes_c111,
    contribuicoes_c120,
    contribuicoes_c170,
    contribuicoes_c180,
    contribuicoes_c181,
    contribuicoes_c185,
    contribuicoes_c188,
    contribuicoes_c190,
    contribuicoes_c191,
    contribuicoes_c195,
    contribuicoes_c198,
    contribuicoes_c199,
    contribuicoes_c380,
    contribuicoes_c381,
    contribuicoes_c385,
    contribuicoes_c395,
    contribuicoes_c396,
    contribuicoes_c400,
    contribuicoes_c405,
    contribuicoes_c481,
    contribuicoes_c485,
    contribuicoes_c489,
    contribuicoes_c490,
    contribuicoes_c491,
    contribuicoes_c495,
    contribuicoes_c499,
    contribuicoes_c500,
    contribuicoes_c501,
    contribuicoes_c505,
    contribuicoes_c509,
    contribuicoes_c600,
    contribuicoes_c601,
    contribuicoes_c605,
    contribuicoes_c609,
    contribuicoes_c990,
    contribuicoes_d001,
    contribuicoes_d010,
    contribuicoes_d100,
    contribuicoes_d101,
    contribuicoes_d105,
    contribuicoes_d111,
    contribuicoes_d200,
    contribuicoes_d201,
    contribuicoes_d205,
    contribuicoes_d209,
    contribuicoes_d300,
    contribuicoes_d309,
    contribuicoes_d350,
    contribuicoes_d359,
    contribuicoes_d500,
    contribuicoes_d501,
    contribuicoes_d505,
    contribuicoes_d509,
    contribuicoes_d600,
    contribuicoes_d601,
    contribuicoes_d605,
    contribuicoes_d609,
    contribuicoes_d990,
    contribuicoes_f001,
    contribuicoes_f010,
    contribuicoes_f100,
    contribuicoes_f111,
    contribuicoes_f120,
    contribuicoes_f129,
    contribuicoes_f130,
    contribuicoes_f139,
    contribuicoes_f150,
    contribuicoes_f200,
    contribuicoes_f205,
    contribuicoes_f210,
    contribuicoes_f211,
    contribuicoes_f500,
    contribuicoes_f509,
    contribuicoes_f510,
    contribuicoes_f519,
    contribuicoes_f525,
    contribuicoes_f550,
    contribuicoes_f559,
    contribuicoes_f560,
    contribuicoes_f569,
    contribuicoes_f600,
    contribuicoes_f700,
    contribuicoes_f800,
    contribuicoes_f990,
    contribuicoes_i001,
    contribuicoes_i010,
    contribuicoes_i100,
    contribuicoes_i199,
    contribuicoes_i200,
    contribuicoes_i299,
    contribuicoes_i300,
    contribuicoes_i399,
    contribuicoes_i990,
    contribuicoes_m001,
    contribuicoes_m100,
    contribuicoes_m105,
    contribuicoes_m110,
    contribuicoes_m200,
    contribuicoes_m205,
    contribuicoes_m210,
    contribuicoes_m211,
    contribuicoes_m220,
    contribuicoes_m230,
    contribuicoes_m300,
    contribuicoes_m350,
    contribuicoes_m400,
    contribuicoes_m410,
    contribuicoes_m500,
    contribuicoes_m505,
    contribuicoes_m510,
    contribuicoes_m600,
    contribuicoes_m605,
    contribuicoes_m610,
    contribuicoes_m611,
    contribuicoes_m620,
    contribuicoes_m630,
    contribuicoes_m700,
    contribuicoes_m800,
    contribuicoes_m810,
    contribuicoes_m990,
    contribuicoes_p001,
    contribuicoes_p010,
    contribuicoes_p100,
    contribuicoes_p110,
    contribuicoes_p199,
    contribuicoes_p200,
    contribuicoes_p210,
    contribuicoes_p990,
    files,
    reg_0000,
    reg_0001,
    reg_0005,
    reg_0015,
    reg_0100,
    reg_0150,
    reg_0175,
    reg_0190,
    reg_0200,
    reg_0205,
    reg_0206,
    reg_0210,
    reg_0220,
    reg_0300,
    reg_0305,
    reg_0400,
    reg_0450,
    reg_0460,
    reg_0500,
    reg_0600,
    reg_0990,
    reg_1001,
    reg_1010,
    reg_1100,
    reg_1105,
    reg_1110,
    reg_1200,
    reg_1210,
    reg_1300,
    reg_1310,
    reg_1320,
    reg_1350,
    reg_1360,
    reg_1370,
    reg_1390,
    reg_1391,
    reg_1400,
    reg_1500,
    reg_1510,
    reg_1600,
    reg_1700,
    reg_1710,
    reg_1800,
    reg_1900,
    reg_1910,
    reg_1920,
    reg_1921,
    reg_1922,
    reg_1923,
    reg_1925,
    reg_1926,
    reg_1990,
    reg_9001,
    reg_9900,
    reg_9990,
    reg_9999,
    reg_c001,
    reg_c100,
    reg_c105,
    reg_c110,
    reg_c111,
    reg_c112,
    reg_c113,
    reg_c114,
    reg_c115,
    reg_c116,
    reg_c120,
    reg_c130,
    reg_c140,
    reg_c141,
    reg_c160,
    reg_c165,
    reg_c170,
    reg_c171,
    reg_c172,
    reg_c173,
    reg_c174,
    reg_c175,
    reg_c176,
    reg_c177,
    reg_c178,
    reg_c179,
    reg_c190,
    reg_c195,
    reg_c197,
    reg_c300,
    reg_c310,
    reg_c320,
    reg_c321,
    reg_c350,
    reg_c370,
    reg_c390,
    reg_c400,
    reg_c405,
    reg_c410,
    reg_c420,
    reg_c425,
    reg_c460,
    reg_c465,
    reg_c470,
    reg_c490,
    reg_c495,
    reg_c500,
    reg_c510,
    reg_c590,
    reg_c600,
    reg_c601,
    reg_c610,
    reg_c690,
    reg_c700,
    reg_c790,
    reg_c791,
    reg_c800,
    reg_c850,
    reg_c860,
    reg_c890,
    reg_c990,
    reg_d001,
    reg_d100,
    reg_d110,
    reg_d120,
    reg_d130,
    reg_d140,
    reg_d150,
    reg_d160,
    reg_d161,
    reg_d162,
    reg_d170,
    reg_d180,
    reg_d190,
    reg_d195,
    reg_d197,
    reg_d300,
    reg_d301,
    reg_d310,
    reg_d350,
    reg_d355,
    reg_d360,
    reg_d365,
    reg_d370,
    reg_d390,
    reg_d400,
    reg_d410,
    reg_d411,
    reg_d420,
    reg_d500,
    reg_d510,
    reg_d530,
    reg_d590,
    reg_d600,
    reg_d610,
    reg_d690,
    reg_d695,
    reg_d696,
    reg_d697,
    reg_d990,
    reg_e001,
    reg_e100,
    reg_e110,
    reg_e111,
    reg_e112,
    reg_e113,
    reg_e115,
    reg_e116,
    reg_e200,
    reg_e210,
    reg_e220,
    reg_e230,
    reg_e240,
    reg_e250,
    reg_e500,
    reg_e510,
    reg_e520,
    reg_e530,
    reg_e990,
    reg_g001,
    reg_g110,
    reg_g125,
    reg_g126,
    reg_g130,
    reg_g140,
    reg_g990,
    reg_h001,
    reg_h005,
    reg_h010,
    reg_h020,
    reg_h990,
    reg_k001,
    reg_k100,
    reg_k200,
    reg_k220,
    reg_k230,
    reg_k235,
    reg_k250,
    reg_k255,
    reg_k990,
);
