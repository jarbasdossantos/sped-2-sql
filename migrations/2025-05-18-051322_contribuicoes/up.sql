CREATE TABLE contribuicoes_0000 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_ver TEXT DEFAULT NULL,
    tipo_escrit TEXT DEFAULT NULL,
    ind_sit_esp TEXT DEFAULT NULL,
    num_rec_anterior TEXT DEFAULT NULL,
    dt_ini DATE DEFAULT NULL,
    dt_fin DATE DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL,
    suframa TEXT DEFAULT NULL,
    ind_nat_pj TEXT DEFAULT NULL,
    ind_ativ TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0035 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_scp TEXT DEFAULT NULL,
    nome_scp TEXT DEFAULT NULL,
    inf_comp TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cpf TEXT DEFAULT NULL,
    crc TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    cep TEXT DEFAULT NULL,
end TEXT DEFAULT NULL,
num TEXT DEFAULT NULL,
compl TEXT DEFAULT NULL,
bairro TEXT DEFAULT NULL,
fone TEXT DEFAULT NULL,
fax TEXT DEFAULT NULL,
email TEXT DEFAULT NULL,
cod_mun TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0110 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inc_trib TEXT DEFAULT NULL,
    ind_apro_cred TEXT DEFAULT NULL,
    cod_tipo_cont TEXT DEFAULT NULL,
    ind_reg_cum TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0111 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    rec_bru_ncum_trib_mi NUMERIC DEFAULT NULL,
    rec_bru_ncum_nt_mi NUMERIC DEFAULT NULL,
    rec_bru_ncum_exp NUMERIC DEFAULT NULL,
    rec_bru_cum NUMERIC DEFAULT NULL,
    rec_bru_total NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_0120 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    mes_dispensa TEXT DEFAULT NULL,
    inf_comp TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0140 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_est TEXT DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    ie TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL,
    im TEXT DEFAULT NULL,
    suframa TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0145 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inc_trib TEXT DEFAULT NULL,
    vl_rec_tot NUMERIC DEFAULT NULL,
    vl_rec_ativ NUMERIC DEFAULT NULL,
    vl_rec_demais_ativ NUMERIC DEFAULT NULL,
    info_compl TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0150 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cod_pais TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    cpf TEXT DEFAULT NULL,
    ie TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL,
    suframa TEXT DEFAULT NULL,
end TEXT DEFAULT NULL,
num TEXT DEFAULT NULL,
compl TEXT DEFAULT NULL,
bairro TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0190 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    descr TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    descr_item TEXT DEFAULT NULL,
    cod_barra TEXT DEFAULT NULL,
    cod_ant_item TEXT DEFAULT NULL,
    unid_inv TEXT DEFAULT NULL,
    tipo_item TEXT DEFAULT NULL,
    cod_ncm TEXT DEFAULT NULL,
    ex_ipi TEXT DEFAULT NULL,
    cod_gen TEXT DEFAULT NULL,
    cod_lst TEXT DEFAULT NULL,
    aliq_icms NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_0205 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    descr_ant_item TEXT DEFAULT NULL,
    dt_ini DATE DEFAULT NULL,
    dt_fim DATE DEFAULT NULL,
    cod_ant_item TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0206 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_comb TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0208 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_tab TEXT DEFAULT NULL,
    cod_gru TEXT DEFAULT NULL,
    marca_com TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0400 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_nat TEXT DEFAULT NULL,
    descr_nat TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0450 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inf TEXT DEFAULT NULL,
    txt TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_alt DATE DEFAULT NULL,
    cod_nat_cc TEXT DEFAULT NULL,
    ind_cta TEXT DEFAULT NULL,
    nivel TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL,
    nome_cta TEXT DEFAULT NULL,
    cod_cta_ref TEXT DEFAULT NULL,
    cnpj_est TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0600 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_alt DATE DEFAULT NULL,
    cod_ccus TEXT DEFAULT NULL,
    ccus TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_0990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_0 TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_1001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_1010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    id_sec_jud TEXT DEFAULT NULL,
    id_vara TEXT DEFAULT NULL,
    ind_nat_acao TEXT DEFAULT NULL,
    desc_dec_jud TEXT DEFAULT NULL,
    dt_sent_jud DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_1020 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_nat_acao TEXT DEFAULT NULL,
    dt_dec_adm DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_1100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    per_apu_cred TEXT DEFAULT NULL,
    orig_cred TEXT DEFAULT NULL,
    cnpj_suc TEXT DEFAULT NULL,
    cod_cred TEXT DEFAULT NULL,
    vl_cred_apu NUMERIC DEFAULT NULL,
    vl_cred_ext_apu NUMERIC DEFAULT NULL,
    vl_tot_cred_apu NUMERIC DEFAULT NULL,
    vl_cred_desc_pa_ant NUMERIC DEFAULT NULL,
    vl_cred_per_pa_ant NUMERIC DEFAULT NULL,
    vl_cred_dcomp_pa_ant NUMERIC DEFAULT NULL,
    sd_cred_disp_efd NUMERIC DEFAULT NULL,
    vl_cred_desc_efd NUMERIC DEFAULT NULL,
    vl_cred_per_efd NUMERIC DEFAULT NULL,
    vl_cred_dcomp_efd NUMERIC DEFAULT NULL,
    vl_cred_trans NUMERIC DEFAULT NULL,
    vl_cred_out NUMERIC DEFAULT NULL,
    sld_cred_fim NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_1300 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_ret TEXT DEFAULT NULL,
    pr_rec_ret TEXT DEFAULT NULL,
    vl_ret_apu NUMERIC DEFAULT NULL,
    vl_ret_ded NUMERIC DEFAULT NULL,
    vl_ret_per NUMERIC DEFAULT NULL,
    vl_ret_dcomp NUMERIC DEFAULT NULL,
    sld_ret NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_1500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    per_apu_cred TEXT DEFAULT NULL,
    orig_cred TEXT DEFAULT NULL,
    cnpj_suc TEXT DEFAULT NULL,
    cod_cred TEXT DEFAULT NULL,
    vl_cred_apu NUMERIC DEFAULT NULL,
    vl_cred_ext_apu NUMERIC DEFAULT NULL,
    vl_tot_cred_apu NUMERIC DEFAULT NULL,
    vl_cred_desc_pa_ant NUMERIC DEFAULT NULL,
    vl_cred_per_pa_ant NUMERIC DEFAULT NULL,
    vl_cred_dcomp_pa_ant NUMERIC DEFAULT NULL,
    sd_cred_disp_efd NUMERIC DEFAULT NULL,
    vl_cred_desc_efd NUMERIC DEFAULT NULL,
    vl_cred_per_efd NUMERIC DEFAULT NULL,
    vl_cred_dcomp_efd NUMERIC DEFAULT NULL,
    vl_cred_trans NUMERIC DEFAULT NULL,
    vl_cred_out NUMERIC DEFAULT NULL,
    sld_cred_fim NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_1700 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_ret TEXT DEFAULT NULL,
    pr_rec_ret TEXT DEFAULT NULL,
    vl_ret_apu NUMERIC DEFAULT NULL,
    vl_ret_ded NUMERIC DEFAULT NULL,
    vl_ret_per NUMERIC DEFAULT NULL,
    vl_ret_dcomp NUMERIC DEFAULT NULL,
    sld_ret NUMERIC DEFAULT NULL
);

CREATE TABLE contribuicoes_1800 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    inc_imob TEXT DEFAULT NULL,
    rec_receb_ret NUMERIC DEFAULT NULL,
    rec_fin_ret NUMERIC DEFAULT NULL,
    bc_ret NUMERIC DEFAULT NULL,
    aliq_ret NUMERIC DEFAULT NULL,
    vl_rec_uni NUMERIC DEFAULT NULL,
    dt_rec_uni DATE DEFAULT NULL,
    cod_rec TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_1900 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub_ser TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    vl_tot_rec NUMERIC DEFAULT NULL,
    quant_doc TEXT DEFAULT NULL,
    cst_pis TEXT DEFAULT NULL,
    cst_cofins TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    info_compl TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_1990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_1 TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_9001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_9900 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    reg_blc TEXT DEFAULT NULL,
    qtd_reg_blc varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_9990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_9 varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_9999 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_a001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_a010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_a100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser varchar DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    chv_nfse varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    dt_exe_serv DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    ind_pgto TEXT DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    vl_pis_ret decimal(21, 2) DEFAULT NULL,
    vl_cofins_ret decimal(21, 2) DEFAULT NULL,
    vl_iss decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_a110 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inf varchar DEFAULT NULL,
    txt_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_a111 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_a120 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    vl_pis_imp decimal(21, 2) DEFAULT NULL,
    dt_pag_pis DATE DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    vl_cofins_imp decimal(21, 2) DEFAULT NULL,
    dt_pag_cofins DATE DEFAULT NULL,
    loc_exe_serv TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_a170 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    descr_compl varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    ind_orig_cred TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(8, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    cod_ccus varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_a990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_a varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    ind_escri TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    chv_nfe varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    dt_e_s DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    ind_pgto TEXT DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_abat_nt decimal(21, 2) DEFAULT NULL,
    vl_merc decimal(21, 2) DEFAULT NULL,
    ind_frt TEXT DEFAULT NULL,
    vl_frt decimal(21, 2) DEFAULT NULL,
    vl_seg decimal(21, 2) DEFAULT NULL,
    vl_out_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_ipi decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    vl_pis_st decimal(21, 2) DEFAULT NULL,
    vl_cofins_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c110 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inf varchar DEFAULT NULL,
    txt_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c111 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c120 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_doc_imp TEXT DEFAULT NULL,
    num_doc_imp varchar DEFAULT NULL,
    vl_pis_imp decimal(21, 2) DEFAULT NULL,
    vl_cofins_imp decimal(21, 2) DEFAULT NULL,
    num_acdraw varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c170 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item varchar DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    descr_compl varchar DEFAULT NULL,
    qtd decimal(24, 5) DEFAULT NULL,
    unid varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL,
    cst_icms varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_nat varchar DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    aliq_st decimal(8, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    ind_apur TEXT DEFAULT NULL,
    cst_ipi varchar DEFAULT NULL,
    cod_enq varchar DEFAULT NULL,
    vl_bc_ipi decimal(21, 2) DEFAULT NULL,
    aliq_ipi decimal(8, 2) DEFAULT NULL,
    vl_ipi decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c180 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    dt_doc_ini DATE DEFAULT NULL,
    dt_doc_fin DATE DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cod_ncm varchar DEFAULT NULL,
    ex_ipi varchar DEFAULT NULL,
    vl_tot_item decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c181 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c185 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c188 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c190 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    dt_ref_ini DATE DEFAULT NULL,
    dt_ref_fin DATE DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cod_ncm varchar DEFAULT NULL,
    ex_ipi varchar DEFAULT NULL,
    vl_tot_item decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c191 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj_cpf_part varchar DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c195 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj_cpf_part varchar DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c198 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c199 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_doc_imp TEXT DEFAULT NULL,
    num_doc_imp varchar DEFAULT NULL,
    vl_pis_imp decimal(21, 2) DEFAULT NULL,
    vl_cofins_imp decimal(21, 2) DEFAULT NULL,
    num_acdraw varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c380 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    dt_doc_ini DATE DEFAULT NULL,
    dt_doc_fin DATE DEFAULT NULL,
    num_doc_ini varchar DEFAULT NULL,
    num_doc_fin varchar DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_doc_canc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c381 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c385 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c395 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    ser varchar DEFAULT NULL,
    sub_ser varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c396 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c400 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    ecf_mod varchar DEFAULT NULL,
    ecf_fab varchar DEFAULT NULL,
    ecf_cx varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c405 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    cro varchar DEFAULT NULL,
    crz varchar DEFAULT NULL,
    num_coo_fin varchar DEFAULT NULL,
    gt_fin decimal(21, 2) DEFAULT NULL,
    vl_brt decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c481 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c485 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c489 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c490 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_doc_ini DATE DEFAULT NULL,
    dt_doc_fin DATE DEFAULT NULL,
    cod_mod varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c491 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c495 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c499 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    dt_e_s DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_inf varchar DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c501 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c505 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c509 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c600 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_mun varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    cod_cons varchar DEFAULT NULL,
    qtd_cons varchar DEFAULT NULL,
    qtd_canc varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cons varchar DEFAULT NULL,
    vl_forn decimal(21, 2) DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_terc decimal(21, 2) DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_c601 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c605 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_c609 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_c990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_c varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    chv_cte varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    dt_a_p DATE DEFAULT NULL,
    tp_cte TEXT DEFAULT NULL,
    chv_cte_ref varchar DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    ind_frt TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_nt decimal(21, 2) DEFAULT NULL,
    cod_inf varchar DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d101 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_frt TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d105 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_frt TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d111 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc_ini varchar DEFAULT NULL,
    num_doc_fin varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    dt_ref DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_d201 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d205 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d209 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d300 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc_ini varchar DEFAULT NULL,
    num_doc_fin varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    dt_ref DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d309 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d350 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    ecf_mod varchar DEFAULT NULL,
    ecf_fab varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    cro varchar DEFAULT NULL,
    crz varchar DEFAULT NULL,
    num_coo_fin varchar DEFAULT NULL,
    gt_fin decimal(21, 2) DEFAULT NULL,
    vl_brt decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d359 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_sit varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    dt_doc DATE DEFAULT NULL,
    dt_a_p DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_terc decimal(21, 2) DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_inf varchar DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_d501 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d505 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d509 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d600 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cod_mun varchar DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub varchar DEFAULT NULL,
    ind_rec TEXT DEFAULT NULL,
    qtd_cons varchar DEFAULT NULL,
    dt_doc_ini DATE DEFAULT NULL,
    dt_doc_fin DATE DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_terc decimal(21, 2) DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_d601 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d605 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_d609 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_d990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_d varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    cod_part varchar DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    dt_oper DATE DEFAULT NULL,
    vl_oper decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(23, 4) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(23, 4) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    ind_orig_cred TEXT DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    cod_ccus varchar DEFAULT NULL,
    desc_doc_oper varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f111 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f120 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    ident_bem_imob varchar DEFAULT NULL,
    ind_orig_cred TEXT DEFAULT NULL,
    ind_util_bem_imob TEXT DEFAULT NULL,
    vl_oper_dep decimal(21, 2) DEFAULT NULL,
    parc_oper_nao_bc_cred decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    cod_ccus varchar DEFAULT NULL,
    desc_bem_imob varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f129 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f130 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    ident_bem_imob varchar DEFAULT NULL,
    ind_orig_cred TEXT DEFAULT NULL,
    ind_util_bem_imob TEXT DEFAULT NULL,
    mes_oper_aquis varchar DEFAULT NULL,
    vl_oper_aquis decimal(21, 2) DEFAULT NULL,
    parc_oper_nao_bc_cred decimal(21, 2) DEFAULT NULL,
    vl_bc_cred decimal(21, 2) DEFAULT NULL,
    ind_nr_parc TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    cod_ccus varchar DEFAULT NULL,
    desc_bem_imob varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f139 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f150 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    vl_tot_est decimal(21, 2) DEFAULT NULL,
    est_imp decimal(21, 2) DEFAULT NULL,
    vl_bc_est decimal(21, 2) DEFAULT NULL,
    vl_bc_men_est decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_cred_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cred_cofins decimal(21, 2) DEFAULT NULL,
    desc_est varchar DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper varchar DEFAULT NULL,
    unid_imob varchar DEFAULT NULL,
    ident_emp varchar DEFAULT NULL,
    desc_unid_imob varchar DEFAULT NULL,
    num_cont varchar DEFAULT NULL,
    cpf_cnpj_adqu varchar DEFAULT NULL,
    dt_oper DATE DEFAULT NULL,
    vl_tot_vend decimal(21, 2) DEFAULT NULL,
    vl_rec_acum decimal(21, 2) DEFAULT NULL,
    vl_tot_rec decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    perc_rec_receb decimal(8, 2) DEFAULT NULL,
    ind_nat_emp TEXT DEFAULT NULL,
    inf_comp varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f205 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_cus_inc_acum_ant decimal(21, 2) DEFAULT NULL,
    vl_cus_inc_per_esc decimal(21, 2) DEFAULT NULL,
    vl_cus_inc_acum decimal(21, 2) DEFAULT NULL,
    vl_exc_bc_cus_inc_acum decimal(21, 2) DEFAULT NULL,
    vl_bc_cus_inc decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_cred_pis_acum decimal(21, 2) DEFAULT NULL,
    vl_cred_pis_desc_ant decimal(21, 2) DEFAULT NULL,
    vl_cred_pis_desc decimal(21, 2) DEFAULT NULL,
    vl_cred_pis_desc_fut decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cred_cofins_acum decimal(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc_ant decimal(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc decimal(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc_fut decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_f210 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_cus_orc decimal(21, 2) DEFAULT NULL,
    vl_exc decimal(21, 2) DEFAULT NULL,
    vl_cus_orc_aju decimal(21, 2) DEFAULT NULL,
    vl_bc_cred decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_cred_pis_util decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cred_cofins_util decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_f211 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec_caixa decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_desc_pis decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_desc_cofins decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f509 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f510 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec_caixa decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_desc_pis decimal(21, 2) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_desc_cofins decimal(21, 2) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f519 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f525 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec decimal(21, 2) DEFAULT NULL,
    ind_rec varchar DEFAULT NULL,
    cnpj_cpf varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    cod_item varchar DEFAULT NULL,
    vl_rec_det decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL,
    cod_cta varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f550 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec_comp decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_desc_pis decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_desc_cofins decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f559 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f560 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec_comp decimal(21, 2) DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_desc_pis decimal(21, 2) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(12, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_desc_cofins decimal(21, 2) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(12, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_mod varchar DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f569 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f600 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_ret varchar DEFAULT NULL,
    dt_ret DATE DEFAULT NULL,
    vl_bc_ret decimal(23, 4) DEFAULT NULL,
    vl_ret decimal(21, 2) DEFAULT NULL,
    cod_rec TEXT DEFAULT NULL,
    ind_nat_rec TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    vl_ret_pis decimal(21, 2) DEFAULT NULL,
    vl_ret_cofins decimal(21, 2) DEFAULT NULL,
    ind_dec TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_f700 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_ori_ded varchar DEFAULT NULL,
    ind_nat_ded TEXT DEFAULT NULL,
    vl_ded_pis decimal(21, 2) DEFAULT NULL,
    vl_ded_cofins decimal(21, 2) DEFAULT NULL,
    vl_bc_oper decimal(21, 2) DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    inf_comp varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_f800 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_nat_even varchar DEFAULT NULL,
    dt_even DATE DEFAULT NULL,
    cnpj_suced varchar DEFAULT NULL,
    pa_cont_cred varchar DEFAULT NULL,
    cod_cred varchar DEFAULT NULL,
    vl_cred_pis decimal(21, 2) DEFAULT NULL,
    vl_cred_cofins decimal(21, 2) DEFAULT NULL,
    per_cred_cis decimal(8, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_f990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_f varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_i001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_i010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    ind_ativ varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_i100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_rec_fin decimal(21, 2) DEFAULT NULL,
    cst varchar DEFAULT NULL,
    vl_tot_ded_ger decimal(21, 2) DEFAULT NULL,
    vl_tot_ded_esp decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(10, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(10, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    inf_comp varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_i199 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_i200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_campo varchar DEFAULT NULL,
    cod_det varchar DEFAULT NULL,
    vl_det decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    inf_comp varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_i299 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_i300 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_comp varchar DEFAULT NULL,
    vl_comp decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    inf_comp varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_i399 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_i990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_i varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_m100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cred varchar DEFAULT NULL,
    ind_cred_ori TEXT DEFAULT NULL,
    vl_bc_cred decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_cred decimal(21, 2) DEFAULT NULL,
    vl_ajus_acres decimal(21, 2) DEFAULT NULL,
    vl_ajus_reduc decimal(21, 2) DEFAULT NULL,
    vl_cred_dif decimal(21, 2) DEFAULT NULL,
    vl_cred_disp decimal(21, 2) DEFAULT NULL,
    ind_desc_cred TEXT DEFAULT NULL,
    vl_cred_desc decimal(21, 2) DEFAULT NULL,
    sld_cred decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m105 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_bc_pis_tot decimal(21, 2) DEFAULT NULL,
    vl_bc_pis_cum decimal(21, 2) DEFAULT NULL,
    vl_bc_pis_nc decimal(21, 2) DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    quant_bc_pis_tot decimal(22, 3) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    desc_cred varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m110 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    descr_aj varchar DEFAULT NULL,
    dt_ref DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_cont_nc_per decimal(21, 2) DEFAULT NULL,
    vl_tot_cred_desc decimal(21, 2) DEFAULT NULL,
    vl_tot_cred_desc_ant decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_nc_dev decimal(21, 2) DEFAULT NULL,
    vl_ret_nc decimal(21, 2) DEFAULT NULL,
    vl_out_ded_nc decimal(21, 2) DEFAULT NULL,
    vl_cont_nc_rec decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_cum_per decimal(21, 2) DEFAULT NULL,
    vl_ret_cum decimal(21, 2) DEFAULT NULL,
    vl_out_ded_cum decimal(21, 2) DEFAULT NULL,
    vl_cont_cum_rec decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_rec decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m205 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_campo varchar DEFAULT NULL,
    cod_rec varchar DEFAULT NULL,
    vl_debito decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m210 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cont varchar DEFAULT NULL,
    vl_rec_brt decimal(21, 2) DEFAULT NULL,
    vl_bc_cont decimal(21, 2) DEFAULT NULL,
    aliq_pis decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_quant decimal(23, 4) DEFAULT NULL,
    vl_cont_apur decimal(21, 2) DEFAULT NULL,
    vl_ajus_acres decimal(21, 2) DEFAULT NULL,
    vl_ajus_reduc decimal(21, 2) DEFAULT NULL,
    vl_cont_difer decimal(21, 2) DEFAULT NULL,
    vl_cont_difer_ant decimal(21, 2) DEFAULT NULL,
    vl_cont_per decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m211 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_tip_coop varchar DEFAULT NULL,
    vl_bc_cont_ant_exc_coop decimal(21, 2) DEFAULT NULL,
    vl_exc_coop_ger decimal(21, 2) DEFAULT NULL,
    vl_exc_esp_coop decimal(21, 2) DEFAULT NULL,
    vl_bc_cont decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m220 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    descr_aj varchar DEFAULT NULL,
    dt_ref DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m230 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    vl_vend decimal(21, 2) DEFAULT NULL,
    vl_nao_receb decimal(21, 2) DEFAULT NULL,
    vl_cont_dif decimal(21, 2) DEFAULT NULL,
    vl_cred_dif decimal(21, 2) DEFAULT NULL,
    cod_cred varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m300 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cont varchar DEFAULT NULL,
    vl_cont_apur_difer decimal(21, 2) DEFAULT NULL,
    nat_cred_desc varchar DEFAULT NULL,
    vl_cred_desc_difer decimal(21, 2) DEFAULT NULL,
    vl_cont_difer_ant decimal(21, 2) DEFAULT NULL,
    per_apur varchar DEFAULT NULL,
    dt_receb DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m350 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_fol decimal(21, 2) DEFAULT NULL,
    vl_exc_bc decimal(21, 2) DEFAULT NULL,
    vl_tot_bc decimal(21, 2) DEFAULT NULL,
    aliq_pis_fol decimal(8, 2) DEFAULT NULL,
    vl_tot_cont_fol decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m400 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_pis varchar DEFAULT NULL,
    vl_tot_rec decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    desc_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m410 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_rec varchar DEFAULT NULL,
    vl_rec decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    desc_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m500 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cred varchar DEFAULT NULL,
    ind_cred_ori TEXT DEFAULT NULL,
    vl_bc_cred decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cred decimal(21, 2) DEFAULT NULL,
    vl_ajus_acres decimal(21, 2) DEFAULT NULL,
    vl_ajus_reduc decimal(21, 2) DEFAULT NULL,
    vl_cred_dif decimal(21, 2) DEFAULT NULL,
    vl_cred_disp decimal(21, 2) DEFAULT NULL,
    ind_desc_cred TEXT DEFAULT NULL,
    vl_cred_desc decimal(21, 2) DEFAULT NULL,
    sld_cred decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m505 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_bc_cred varchar DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_bc_cofins_tot decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins_cum decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins_nc decimal(21, 2) DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    quant_bc_cofins_tot decimal(22, 3) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    desc_cred varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m510 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    descr_aj varchar DEFAULT NULL,
    dt_ref DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m600 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_cont_nc_per decimal(21, 2) DEFAULT NULL,
    vl_tot_cred_desc decimal(21, 2) DEFAULT NULL,
    vl_tot_cred_desc_ant decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_nc_dev decimal(21, 2) DEFAULT NULL,
    vl_ret_nc decimal(21, 2) DEFAULT NULL,
    vl_out_ded_nc decimal(21, 2) DEFAULT NULL,
    vl_cont_nc_rec decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_cum_per decimal(21, 2) DEFAULT NULL,
    vl_ret_cum decimal(21, 2) DEFAULT NULL,
    vl_out_ded_cum decimal(21, 2) DEFAULT NULL,
    vl_cont_cum_rec decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_rec decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m605 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_campo varchar DEFAULT NULL,
    cod_rec varchar DEFAULT NULL,
    vl_debito decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m610 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cont varchar DEFAULT NULL,
    vl_rec_brt decimal(21, 2) DEFAULT NULL,
    vl_bc_cont decimal(21, 2) DEFAULT NULL,
    aliq_cofins decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_quant decimal(23, 4) DEFAULT NULL,
    vl_cont_apur decimal(21, 2) DEFAULT NULL,
    vl_ajus_acres decimal(21, 2) DEFAULT NULL,
    vl_ajus_reduc decimal(21, 2) DEFAULT NULL,
    vl_cont_difer decimal(21, 2) DEFAULT NULL,
    vl_cont_difer_ant decimal(21, 2) DEFAULT NULL,
    vl_cont_per decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m611 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_tip_coop varchar DEFAULT NULL,
    vl_bc_cont_ant_exc_coop decimal(21, 2) DEFAULT NULL,
    vl_exc_coop_ger decimal(21, 2) DEFAULT NULL,
    vl_exc_esp_coop decimal(21, 2) DEFAULT NULL,
    vl_bc_cont decimal(21, 2) DEFAULT NULL
);

CREATE TABLE contribuicoes_m620 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    descr_aj varchar DEFAULT NULL,
    dt_ref DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m630 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL,
    vl_vend decimal(21, 2) DEFAULT NULL,
    vl_nao_receb decimal(21, 2) DEFAULT NULL,
    vl_cont_dif decimal(21, 2) DEFAULT NULL,
    vl_cred_dif decimal(21, 2) DEFAULT NULL,
    cod_cred varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m700 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_cont varchar DEFAULT NULL,
    vl_cont_apur_difer decimal(21, 2) DEFAULT NULL,
    nat_bc_cred_desc varchar DEFAULT NULL,
    vl_cred_desc_difer decimal(21, 2) DEFAULT NULL,
    vl_cont_difer_ant decimal(21, 2) DEFAULT NULL,
    per_apur varchar DEFAULT NULL,
    dt_receb DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_m800 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_cofins varchar DEFAULT NULL,
    vl_tot_rec decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    desc_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m810 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nat_rec varchar DEFAULT NULL,
    vl_rec decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    desc_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_m990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_m varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_p001 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_p010 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cnpj varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_p100 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini DATE DEFAULT NULL,
    dt_fim DATE DEFAULT NULL,
    vl_rec_tot_est decimal(21, 2) DEFAULT NULL,
    cod_ativ_econ varchar DEFAULT NULL,
    vl_rec_ativ_estab decimal(21, 2) DEFAULT NULL,
    vl_exc decimal(21, 2) DEFAULT NULL,
    vl_bc_cont decimal(21, 2) DEFAULT NULL,
    aliq_cont decimal(12, 4) DEFAULT NULL,
    vl_cont_apu decimal(21, 2) DEFAULT NULL,
    cod_cta varchar DEFAULT NULL,
    info_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_p110 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_campo varchar DEFAULT NULL,
    cod_det varchar DEFAULT NULL,
    det_valor decimal(21, 2) DEFAULT NULL,
    inf_compl varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_p199 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_proc varchar DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL
);

CREATE TABLE contribuicoes_p200 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    per_ref varchar DEFAULT NULL,
    vl_tot_cont_apu decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_reduc decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_acres decimal(21, 2) DEFAULT NULL,
    vl_tot_cont_dev decimal(21, 2) DEFAULT NULL,
    cod_rec varchar DEFAULT NULL
);

CREATE TABLE contribuicoes_p210 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj varchar DEFAULT NULL,
    num_doc varchar DEFAULT NULL,
    descr_aj varchar DEFAULT NULL,
    dt_ref DATE DEFAULT NULL
);

CREATE TABLE contribuicoes_p990 (
    id INTEGER PRIMARY KEY,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_p varchar DEFAULT NULL
);