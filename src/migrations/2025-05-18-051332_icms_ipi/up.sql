CREATE TABLE reg_0000 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_ver TEXT DEFAULT NULL,
    cod_fin TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    cpf TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    ie TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL,
    im TEXT DEFAULT NULL,
    suframa TEXT DEFAULT NULL,
    ind_perfil TEXT DEFAULT NULL,
    ind_ativ TEXT DEFAULT NULL
);

CREATE TABLE reg_0001 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE reg_0005 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    fantasia TEXT DEFAULT NULL,
    cep TEXT DEFAULT NULL,
    endereco TEXT DEFAULT NULL,
    num TEXT DEFAULT NULL,
    compl TEXT DEFAULT NULL,
    bairro TEXT DEFAULT NULL,
    fone TEXT DEFAULT NULL,
    fax TEXT DEFAULT NULL,
    email TEXT DEFAULT NULL
);

CREATE TABLE reg_0015 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    uf_st TEXT DEFAULT NULL,
    ie_st TEXT DEFAULT NULL
);

CREATE TABLE reg_0100 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    nome TEXT DEFAULT NULL,
    cpf TEXT DEFAULT NULL,
    crc TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    cep TEXT DEFAULT NULL,
    endereco TEXT DEFAULT NULL,
    num TEXT DEFAULT NULL,
    compl TEXT DEFAULT NULL,
    bairro TEXT DEFAULT NULL,
    fone TEXT DEFAULT NULL,
    fax TEXT DEFAULT NULL,
    email TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL
);

CREATE TABLE reg_0150 (
    id INTEGER PRIMARY KEY NOT NULL,
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
    endereco TEXT DEFAULT NULL,
    num TEXT DEFAULT NULL,
    compl TEXT DEFAULT NULL,
    bairro TEXT DEFAULT NULL
);

CREATE TABLE reg_0175 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_alt date DEFAULT NULL,
    nr_campo TEXT DEFAULT NULL,
    cont_ant TEXT DEFAULT NULL
);

CREATE TABLE reg_0190 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    descr TEXT DEFAULT NULL
);

CREATE TABLE reg_0210 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item_comp TEXT DEFAULT NULL,
    qtd_comp decimal(17, 6) DEFAULT NULL,
    perda decimal(5, 2) DEFAULT NULL
);

CREATE TABLE reg_0220 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    unid_conv TEXT DEFAULT NULL,
    fat_conv decimal(25, 6) DEFAULT NULL
);

CREATE TABLE reg_0300 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_ind_bem TEXT DEFAULT NULL,
    ident_merc TEXT DEFAULT NULL,
    descr_item TEXT DEFAULT NULL,
    cod_prnc TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL,
    nr_parc TEXT DEFAULT NULL
);

CREATE TABLE reg_0305 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_ccus TEXT DEFAULT NULL,
    func TEXT DEFAULT NULL,
    vida_util TEXT DEFAULT NULL
);

CREATE TABLE reg_0460 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL,
    txt TEXT DEFAULT NULL
);

CREATE TABLE reg_0500 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_alt date DEFAULT NULL,
    cod_nat_cc TEXT DEFAULT NULL,
    ind_cta TEXT DEFAULT NULL,
    nivel TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL,
    nome_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_1010 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_exp TEXT DEFAULT NULL,
    ind_ccrf TEXT DEFAULT NULL,
    ind_comb TEXT DEFAULT NULL,
    ind_usina TEXT DEFAULT NULL,
    ind_va TEXT DEFAULT NULL,
    ind_ee TEXT DEFAULT NULL,
    ind_cart TEXT DEFAULT NULL,
    ind_form TEXT DEFAULT NULL,
    ind_aer TEXT DEFAULT NULL
);

CREATE TABLE reg_1100 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_doc TEXT DEFAULT NULL,
    nro_de TEXT DEFAULT NULL,
    dt_de date DEFAULT NULL,
    nat_exp TEXT DEFAULT NULL,
    nro_re TEXT DEFAULT NULL,
    dt_re date DEFAULT NULL,
    chc_emb TEXT DEFAULT NULL,
    dt_chc date DEFAULT NULL,
    dt_avb date DEFAULT NULL,
    tp_chc TEXT DEFAULT NULL,
    pais TEXT DEFAULT NULL
);

CREATE TABLE reg_1105 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    chv_nfe TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL
);

CREATE TABLE reg_1110 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    chv_nfe TEXT DEFAULT NULL,
    nr_memo TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL
);

CREATE TABLE reg_1200 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj_apur TEXT DEFAULT NULL,
    sld_cred decimal(21, 2) DEFAULT NULL,
    cred_apr decimal(21, 2) DEFAULT NULL,
    cred_receb decimal(21, 2) DEFAULT NULL,
    cred_util decimal(21, 2) DEFAULT NULL,
    sld_cred_fim decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1210 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    tipo_util TEXT DEFAULT NULL,
    nr_doc TEXT DEFAULT NULL,
    vl_cred_util decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1300 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    dt_fech date DEFAULT NULL,
    estq_abert decimal(22, 3) DEFAULT NULL,
    vol_entr decimal(22, 3) DEFAULT NULL,
    vol_disp decimal(22, 3) DEFAULT NULL,
    vol_saidas decimal(22, 3) DEFAULT NULL,
    estq_escr decimal(22, 3) DEFAULT NULL,
    val_aj_perda decimal(22, 3) DEFAULT NULL,
    val_aj_ganho decimal(22, 3) DEFAULT NULL,
    fech_fisico decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_1310 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_tanque TEXT DEFAULT NULL,
    estq_abert decimal(22, 3) DEFAULT NULL,
    vol_entr decimal(22, 3) DEFAULT NULL,
    vol_disp decimal(22, 3) DEFAULT NULL,
    vol_saidas decimal(22, 3) DEFAULT NULL,
    estq_escr decimal(22, 3) DEFAULT NULL,
    val_aj_perda decimal(22, 3) DEFAULT NULL,
    val_aj_ganho decimal(22, 3) DEFAULT NULL,
    fech_fisico decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_1320 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_bico TEXT DEFAULT NULL,
    nr_interv TEXT DEFAULT NULL,
    mot_interv TEXT DEFAULT NULL,
    nom_interv TEXT DEFAULT NULL,
    cnpj_interv TEXT DEFAULT NULL,
    cpf_interv TEXT DEFAULT NULL,
    val_fecha decimal(22, 3) DEFAULT NULL,
    val_abert decimal(22, 3) DEFAULT NULL,
    vol_aferi decimal(22, 3) DEFAULT NULL,
    vol_vendas decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_1350 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    serie TEXT DEFAULT NULL,
    fabricante TEXT DEFAULT NULL,
    modelo TEXT DEFAULT NULL,
    tipo_medicao TEXT DEFAULT NULL
);

CREATE TABLE reg_1360 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_lacre TEXT DEFAULT NULL,
    dat_aplicacao date DEFAULT NULL
);

CREATE TABLE reg_1370 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_bico TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    num_tanque TEXT DEFAULT NULL
);

CREATE TABLE reg_1390 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_prod TEXT DEFAULT NULL
);

CREATE TABLE reg_1391 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_registro date DEFAULT NULL,
    qtd_moid decimal(21, 2) DEFAULT NULL,
    estq_ini decimal(21, 2) DEFAULT NULL,
    qtd_produz decimal(21, 2) DEFAULT NULL,
    ent_anid_hid decimal(21, 2) DEFAULT NULL,
    outr_entr decimal(21, 2) DEFAULT NULL,
    perda decimal(21, 2) DEFAULT NULL,
    cons decimal(21, 2) DEFAULT NULL,
    sai_ani_hid decimal(21, 2) DEFAULT NULL,
    saidas decimal(21, 2) DEFAULT NULL,
    estq_fin decimal(21, 2) DEFAULT NULL,
    estq_ini_mel decimal(21, 2) DEFAULT NULL,
    prod_dia_mel decimal(21, 2) DEFAULT NULL,
    util_mel decimal(21, 2) DEFAULT NULL,
    prod_alc_mel decimal(21, 2) DEFAULT NULL,
    obs TEXT DEFAULT NULL
);

CREATE TABLE reg_1400 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    mun TEXT DEFAULT NULL,
    valor decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1500 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    cod_cons TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    dt_e_s date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_forn decimal(21, 2) DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_terc decimal(21, 2) DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    cod_inf TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofis decimal(21, 2) DEFAULT NULL,
    tp_ligacao TEXT DEFAULT NULL,
    cod_grupo_tensao TEXT DEFAULT NULL
);

CREATE TABLE reg_1510 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    aliq_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    ind_rec TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofis decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_1600 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    tot_credito decimal(21, 2) DEFAULT NULL,
    tot_debito decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1700 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_disp TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc_ini TEXT DEFAULT NULL,
    num_doc_fin TEXT DEFAULT NULL,
    num_aut TEXT DEFAULT NULL
);

CREATE TABLE reg_1710 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_doc_ini TEXT DEFAULT NULL,
    num_doc_fin TEXT DEFAULT NULL
);

CREATE TABLE reg_1800 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_carga decimal(21, 2) DEFAULT NULL,
    vl_pass decimal(21, 2) DEFAULT NULL,
    vl_fat decimal(21, 2) DEFAULT NULL,
    ind_rat decimal(14, 6) DEFAULT NULL,
    vl_icms_ant decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms_apur decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_apur decimal(21, 2) DEFAULT NULL,
    vl_dif decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1900 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_apur_icms TEXT DEFAULT NULL,
    descr_compl_out_apur TEXT DEFAULT NULL
);

CREATE TABLE reg_1910 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL
);

CREATE TABLE reg_1920 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_transf_debitos_oa decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_debitos_oa decimal(21, 2) DEFAULT NULL,
    vl_estornos_cred_oa decimal(21, 2) DEFAULT NULL,
    vl_tot_transf_creditos_oa decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_creditos_oa decimal(21, 2) DEFAULT NULL,
    vl_estornos_deb_oa decimal(21, 2) DEFAULT NULL,
    vl_sld_credor_ant_oa decimal(21, 2) DEFAULT NULL,
    vl_sld_apurado_oa decimal(21, 2) DEFAULT NULL,
    vl_tot_ded decimal(21, 2) DEFAULT NULL,
    vl_icms_recolher_oa decimal(21, 2) DEFAULT NULL,
    vl_sld_credor_transp_oa decimal(21, 2) DEFAULT NULL,
    deb_esp_oa decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1921 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj_apur TEXT DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL,
    vl_aj_apur decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1922 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_da TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_1923 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_aj_item decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_1925 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inf_adic TEXT DEFAULT NULL,
    vl_inf_adic decimal(21, 2) DEFAULT NULL,
    desc_compl_aj TEXT DEFAULT NULL
);

CREATE TABLE reg_1926 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_or TEXT DEFAULT NULL,
    vl_or decimal(21, 2) DEFAULT NULL,
    dt_vcto date DEFAULT NULL,
    cod_rec TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL,
    mes_ref TEXT DEFAULT NULL
);

CREATE TABLE reg_c105 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    oper TEXT DEFAULT NULL,
    cod_uf TEXT DEFAULT NULL
);

CREATE TABLE reg_c112 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_da TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    num_da TEXT DEFAULT NULL,
    cod_aut TEXT DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    dt_vcto date DEFAULT NULL,
    dt_pgto date DEFAULT NULL
);

CREATE TABLE reg_c113 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL
);

CREATE TABLE reg_c114 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ecf_fab TEXT DEFAULT NULL,
    ecf_cx TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL
);

CREATE TABLE reg_c115 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_carga TEXT DEFAULT NULL,
    cnpj_col TEXT DEFAULT NULL,
    ie_col TEXT DEFAULT NULL,
    cpf_col TEXT DEFAULT NULL,
    cod_mun_col TEXT DEFAULT NULL,
    cnpj_entg TEXT DEFAULT NULL,
    ie_entg TEXT DEFAULT NULL,
    cpf_entg TEXT DEFAULT NULL,
    cod_mun_entg TEXT DEFAULT NULL
);

CREATE TABLE reg_c116 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    nr_sat TEXT DEFAULT NULL,
    chv_cfe TEXT DEFAULT NULL,
    num_cfe TEXT DEFAULT NULL,
    dt_doc TEXT DEFAULT NULL
);

CREATE TABLE reg_c120 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_doc_imp TEXT DEFAULT NULL,
    num_doc_imp TEXT DEFAULT NULL,
    pis_imp decimal(21, 2) DEFAULT NULL,
    cofins_imp decimal(21, 2) DEFAULT NULL,
    num_acdraw TEXT DEFAULT NULL
);

CREATE TABLE reg_c130 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_bc_issqn decimal(21, 2) DEFAULT NULL,
    vl_issqn decimal(21, 2) DEFAULT NULL,
    vl_bc_irrf decimal(21, 2) DEFAULT NULL,
    vl_irrf decimal(21, 2) DEFAULT NULL,
    vl_bc_prev decimal(21, 2) DEFAULT NULL,
    vl_prev decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c140 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    ind_tit TEXT DEFAULT NULL,
    desc_tit TEXT DEFAULT NULL,
    num_tit TEXT DEFAULT NULL,
    qtd_parc TEXT DEFAULT NULL,
    vl_tit decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c141 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_parc TEXT DEFAULT NULL,
    dt_vcto date DEFAULT NULL,
    vl_parc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c160 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    qtd_vol TEXT DEFAULT NULL,
    peso_brt decimal(21, 2) DEFAULT NULL,
    peso_liq decimal(21, 2) DEFAULT NULL,
    uf_id TEXT DEFAULT NULL
);

CREATE TABLE reg_c165 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    cod_aut TEXT DEFAULT NULL,
    nr_passe TEXT DEFAULT NULL,
    hora TEXT DEFAULT NULL,
    temper decimal(20, 1) DEFAULT NULL,
    qtd_vol TEXT DEFAULT NULL,
    peso_brt decimal(21, 2) DEFAULT NULL,
    peso_liq decimal(21, 2) DEFAULT NULL,
    nom_mot TEXT DEFAULT NULL,
    cpf TEXT DEFAULT NULL,
    uf_id TEXT DEFAULT NULL
);

CREATE TABLE reg_c170 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    descr_compl TEXT DEFAULT NULL,
    qtd decimal(24, 5) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cod_nat TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    aliq_st decimal(8, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    ind_apur TEXT DEFAULT NULL,
    cst_ipi TEXT DEFAULT NULL,
    cod_enq TEXT DEFAULT NULL,
    vl_bc_ipi decimal(21, 2) DEFAULT NULL,
    aliq_ipi decimal(8, 2) DEFAULT NULL,
    vl_ipi decimal(21, 2) DEFAULT NULL,
    cst_pis TEXT DEFAULT NULL,
    vl_bc_pis decimal(21, 2) DEFAULT NULL,
    aliq_pis_perc decimal(12, 4) DEFAULT NULL,
    quant_bc_pis decimal(22, 3) DEFAULT NULL,
    aliq_pis_reais decimal(23, 4) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    cst_cofins TEXT DEFAULT NULL,
    vl_bc_cofins decimal(21, 2) DEFAULT NULL,
    aliq_cofins_perc decimal(12, 4) DEFAULT NULL,
    quant_bc_cofins decimal(22, 3) DEFAULT NULL,
    aliq_cofins_reais decimal(23, 4) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_c171 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_tanque TEXT DEFAULT NULL,
    qtde decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_c172 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_bc_issqn decimal(21, 2) DEFAULT NULL,
    aliq_issqn decimal(8, 2) DEFAULT NULL,
    vl_issqn decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c173 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    lote_med TEXT DEFAULT NULL,
    qtd_item decimal(22, 3) DEFAULT NULL,
    dt_fab date DEFAULT NULL,
    dt_val date DEFAULT NULL,
    ind_med TEXT DEFAULT NULL,
    tp_prod TEXT DEFAULT NULL,
    vl_tab_max decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c174 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_arm TEXT DEFAULT NULL,
    num_arm TEXT DEFAULT NULL,
    descr_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_c175 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_veic_oper TEXT DEFAULT NULL,
    cnpj TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    chassi_veic TEXT DEFAULT NULL
);

CREATE TABLE reg_c176 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod_ult_e TEXT DEFAULT NULL,
    num_doc_ult_e TEXT DEFAULT NULL,
    ser_ult_e TEXT DEFAULT NULL,
    dt_ult_e date DEFAULT NULL,
    cod_part_ult_e TEXT DEFAULT NULL,
    quant_ult_e decimal(22, 3) DEFAULT NULL,
    vl_unit_ult_e decimal(22, 3) DEFAULT NULL,
    vl_unit_bc_st decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_c177 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_selo_ipi TEXT DEFAULT NULL,
    qt_selo_ipi TEXT DEFAULT NULL
);

CREATE TABLE reg_c178 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cl_enq TEXT DEFAULT NULL,
    vl_unid decimal(21, 2) DEFAULT NULL,
    quant_pad decimal(22, 3) DEFAULT NULL
);

CREATE TABLE reg_c179 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    bc_st_orig_dest decimal(21, 2) DEFAULT NULL,
    icms_st_rep decimal(21, 2) DEFAULT NULL,
    icms_st_compl decimal(21, 2) DEFAULT NULL,
    bc_ret decimal(21, 2) DEFAULT NULL,
    icms_ret decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c190 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    vl_ipi decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c195 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_c197 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj TEXT DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_outros decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c300 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc_ini TEXT DEFAULT NULL,
    num_doc_fin TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_c310 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_doc_canc TEXT DEFAULT NULL
);

CREATE TABLE reg_c320 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c321 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c350 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub_ser TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cnpj_cpf TEXT DEFAULT NULL,
    vl_merc decimal(21, 2) DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofis decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_c370 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c390 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c410 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c420 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_tot_par TEXT DEFAULT NULL,
    vlr_acum_tot decimal(21, 2) DEFAULT NULL,
    nr_tot TEXT DEFAULT NULL,
    descr_nr_tot TEXT DEFAULT NULL
);

CREATE TABLE reg_c425 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c460 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cpf_cnpj TEXT DEFAULT NULL,
    nome_adq TEXT DEFAULT NULL
);

CREATE TABLE reg_c465 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    chv_cfe TEXT DEFAULT NULL,
    num_ccf TEXT DEFAULT NULL
);

CREATE TABLE reg_c470 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    qtd_canc decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c490 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c495 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    qtd_canc decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_canc decimal(21, 2) DEFAULT NULL,
    vl_acmo decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_isen decimal(21, 2) DEFAULT NULL,
    vl_nt decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c500 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    cod_cons TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    dt_e_s date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_forn decimal(21, 2) DEFAULT NULL,
    vl_serv_nt decimal(21, 2) DEFAULT NULL,
    vl_terc decimal(21, 2) DEFAULT NULL,
    vl_da decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    cod_inf TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    tp_ligacao TEXT DEFAULT NULL,
    cod_grupo_tensao TEXT DEFAULT NULL
);

CREATE TABLE reg_c510 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    aliq_st decimal(8, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    ind_rec TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_c590 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c601 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_doc_canc TEXT DEFAULT NULL
);

CREATE TABLE reg_c610 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_c690 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c700 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    nro_ord_ini TEXT DEFAULT NULL,
    nro_ord_fin TEXT DEFAULT NULL,
    dt_doc_ini date DEFAULT NULL,
    dt_doc_fin date DEFAULT NULL,
    nom_mest TEXT DEFAULT NULL,
    chv_cod_dig TEXT DEFAULT NULL
);

CREATE TABLE reg_c790 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c791 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c800 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    num_cfe TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_cfe decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cnpj_cpf TEXT DEFAULT NULL,
    nr_sat TEXT DEFAULT NULL,
    chv_cfe TEXT DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_merc decimal(21, 2) DEFAULT NULL,
    vl_out_da decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_pis_st decimal(21, 2) DEFAULT NULL,
    vl_cofins_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_c850 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_c860 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    nr_sat TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    doc_ini TEXT DEFAULT NULL,
    doc_fim TEXT DEFAULT NULL
);

CREATE TABLE reg_c890 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d100 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_oper TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    chv_cte TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    dt_a_p date DEFAULT NULL,
    tp_ct_e TEXT DEFAULT NULL,
    chv_cte_ref TEXT DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    ind_frt TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_nt decimal(21, 2) DEFAULT NULL,
    cod_inf TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_d110 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_out decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d120 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    uf_id TEXT DEFAULT NULL
);

CREATE TABLE reg_d130 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part_consg TEXT DEFAULT NULL,
    cod_part_red TEXT DEFAULT NULL,
    ind_frt_red TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    vl_liq_frt decimal(21, 2) DEFAULT NULL,
    vl_sec_cat decimal(21, 2) DEFAULT NULL,
    vl_desp decimal(21, 2) DEFAULT NULL,
    vl_pedg decimal(21, 2) DEFAULT NULL,
    vl_out decimal(21, 2) DEFAULT NULL,
    vl_frt decimal(21, 2) DEFAULT NULL,
    uf_id TEXT DEFAULT NULL
);

CREATE TABLE reg_d140 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part_consg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    ind_veic TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    ind_nav TEXT DEFAULT NULL,
    viagem TEXT DEFAULT NULL,
    vl_frt_liq decimal(21, 2) DEFAULT NULL,
    vl_desp_port decimal(21, 2) DEFAULT NULL,
    vl_desp_car_desc decimal(21, 2) DEFAULT NULL,
    vl_out decimal(21, 2) DEFAULT NULL,
    vl_frt_brt decimal(21, 2) DEFAULT NULL,
    vl_frt_mm decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d150 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    viagem TEXT DEFAULT NULL,
    ind_tfa TEXT DEFAULT NULL,
    vl_peso_tx decimal(21, 2) DEFAULT NULL,
    vl_tx_terr decimal(21, 2) DEFAULT NULL,
    vl_tx_red decimal(21, 2) DEFAULT NULL,
    vl_out decimal(21, 2) DEFAULT NULL,
    vl_tx_adv decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d160 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    despacho TEXT DEFAULT NULL,
    cnpj_cpf_rem TEXT DEFAULT NULL,
    ie_rem TEXT DEFAULT NULL,
    cod_mun_ori TEXT DEFAULT NULL,
    cnpj_cpf_dest TEXT DEFAULT NULL,
    ie_dest TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL
);

CREATE TABLE reg_d161 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_carga TEXT DEFAULT NULL,
    cnpj_cpf_col TEXT DEFAULT NULL,
    ie_col TEXT DEFAULT NULL,
    cod_mun_col TEXT DEFAULT NULL,
    cnpj_cpf_entg TEXT DEFAULT NULL,
    ie_entg TEXT DEFAULT NULL,
    cod_mun_entg TEXT DEFAULT NULL
);

CREATE TABLE reg_d162 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_merc decimal(21, 2) DEFAULT NULL,
    qtd_vol TEXT DEFAULT NULL,
    peso_brt decimal(21, 2) DEFAULT NULL,
    peso_liq decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d170 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part_consg TEXT DEFAULT NULL,
    cod_part_red TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    otm TEXT DEFAULT NULL,
    ind_nat_frt TEXT DEFAULT NULL,
    vl_liq_frt decimal(21, 2) DEFAULT NULL,
    vl_gris decimal(21, 2) DEFAULT NULL,
    vl_pdg decimal(21, 2) DEFAULT NULL,
    vl_out decimal(21, 2) DEFAULT NULL,
    vl_frt decimal(21, 2) DEFAULT NULL,
    veic_id TEXT DEFAULT NULL,
    uf_id TEXT DEFAULT NULL
);

CREATE TABLE reg_d180 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_seq TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cnpj_cpf_emit TEXT DEFAULT NULL,
    uf_emit TEXT DEFAULT NULL,
    ie_emit TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    cnpj_cpf_tom TEXT DEFAULT NULL,
    uf_tom TEXT DEFAULT NULL,
    ie_tom TEXT DEFAULT NULL,
    cod_mun_dest TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d190 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d195 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_d197 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj TEXT DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_outros decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d300 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc_ini TEXT DEFAULT NULL,
    num_doc_fin TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_seg decimal(21, 2) DEFAULT NULL,
    vl_out_desp decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_d301 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_doc_canc TEXT DEFAULT NULL
);

CREATE TABLE reg_d310 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d350 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ecf_mod TEXT DEFAULT NULL,
    ecf_fab TEXT DEFAULT NULL,
    ecf_cx TEXT DEFAULT NULL
);

CREATE TABLE reg_d355 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cro TEXT DEFAULT NULL,
    crz TEXT DEFAULT NULL,
    num_coo_fin TEXT DEFAULT NULL,
    gt_fin decimal(21, 2) DEFAULT NULL,
    vl_brt decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d360 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d365 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_tot_par TEXT DEFAULT NULL,
    vlr_acum_tot decimal(21, 2) DEFAULT NULL,
    nr_tot TEXT DEFAULT NULL,
    descr_nr_tot TEXT DEFAULT NULL
);

CREATE TABLE reg_d370 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    qtd_bilh TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d390 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_issqn decimal(21, 2) DEFAULT NULL,
    aliq_issqn decimal(8, 2) DEFAULT NULL,
    vl_issqn decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d400 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_sit TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    vl_doc decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_d410 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc_ini TEXT DEFAULT NULL,
    num_doc_fin TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d411 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_doc_canc TEXT DEFAULT NULL
);

CREATE TABLE reg_d420 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mun_orig TEXT DEFAULT NULL,
    vl_serv decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_d510 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    ind_rec TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_d530 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_serv TEXT DEFAULT NULL,
    dt_ini_serv date DEFAULT NULL,
    dt_fin_serv date DEFAULT NULL,
    per_fiscal TEXT DEFAULT NULL,
    cod_area TEXT DEFAULT NULL,
    terminal TEXT DEFAULT NULL
);

CREATE TABLE reg_d590 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d600 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    cod_mun TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    cod_cons TEXT DEFAULT NULL,
    qtd_cons TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
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

CREATE TABLE reg_d610 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_class TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    vl_desc decimal(21, 2) DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    vl_pis decimal(21, 2) DEFAULT NULL,
    vl_cofins decimal(21, 2) DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL
);

CREATE TABLE reg_d690 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d695 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    nro_ord_ini TEXT DEFAULT NULL,
    nro_ord_fin TEXT DEFAULT NULL,
    dt_doc_ini date DEFAULT NULL,
    dt_doc_fin date DEFAULT NULL,
    nom_mest TEXT DEFAULT NULL,
    chv_cod_dig TEXT DEFAULT NULL
);

CREATE TABLE reg_d696 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    aliq_icms decimal(8, 2) DEFAULT NULL,
    vl_opr decimal(21, 2) DEFAULT NULL,
    vl_bc_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL,
    vl_red_bc decimal(21, 2) DEFAULT NULL,
    cod_obs TEXT DEFAULT NULL
);

CREATE TABLE reg_d697 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    vl_bc_icms_st decimal(21, 2) DEFAULT NULL,
    vl_icms_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e001 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE reg_e100 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL
);

CREATE TABLE reg_e110 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_tot_debitos decimal(21, 2) DEFAULT NULL,
    vl_aj_debitos decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_debitos decimal(21, 2) DEFAULT NULL,
    vl_estornos_cred decimal(21, 2) DEFAULT NULL,
    vl_tot_creditos decimal(21, 2) DEFAULT NULL,
    vl_aj_creditos decimal(21, 2) DEFAULT NULL,
    vl_tot_aj_creditos decimal(21, 2) DEFAULT NULL,
    vl_estornos_deb decimal(21, 2) DEFAULT NULL,
    vl_sld_credor_ant decimal(21, 2) DEFAULT NULL,
    vl_sld_apurado decimal(21, 2) DEFAULT NULL,
    vl_tot_ded decimal(21, 2) DEFAULT NULL,
    vl_icms_recolher decimal(21, 2) DEFAULT NULL,
    vl_sld_credor_transportar decimal(21, 2) DEFAULT NULL,
    deb_esp decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e111 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj_apur TEXT DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL,
    vl_aj_apur decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e112 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_da TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_e113 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_aj_item decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e115 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_inf_adic TEXT DEFAULT NULL,
    vl_inf_adic decimal(21, 2) DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL
);

CREATE TABLE reg_e116 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_or TEXT DEFAULT NULL,
    vl_or decimal(21, 2) DEFAULT NULL,
    dt_vcto date DEFAULT NULL,
    cod_rec TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL,
    mes_ref TEXT DEFAULT NULL
);

CREATE TABLE reg_e200 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    uf TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL
);

CREATE TABLE reg_e210 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov_st TEXT DEFAULT NULL,
    vl_sld_cred_ant_st decimal(21, 2) DEFAULT NULL,
    vl_devol_st decimal(21, 2) DEFAULT NULL,
    vl_ressarc_st decimal(21, 2) DEFAULT NULL,
    vl_out_cred_st decimal(21, 2) DEFAULT NULL,
    vl_aj_creditos_st decimal(21, 2) DEFAULT NULL,
    vl_retencao_st decimal(21, 2) DEFAULT NULL,
    vl_out_deb_st decimal(21, 2) DEFAULT NULL,
    vl_aj_debitos_st decimal(21, 2) DEFAULT NULL,
    vl_sld_dev_ant_st decimal(21, 2) DEFAULT NULL,
    vl_deducoes_st decimal(21, 2) DEFAULT NULL,
    vl_icms_recol_st decimal(21, 2) DEFAULT NULL,
    vl_sld_cred_st_transportar decimal(21, 2) DEFAULT NULL,
    deb_esp_st decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e220 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_aj_apur TEXT DEFAULT NULL,
    descr_compl_aj TEXT DEFAULT NULL,
    vl_aj_apur decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e230 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_da TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL
);

CREATE TABLE reg_e240 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    ser TEXT DEFAULT NULL,
    sub TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    vl_aj_item decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e250 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_or TEXT DEFAULT NULL,
    vl_or decimal(21, 2) DEFAULT NULL,
    dt_vcto date DEFAULT NULL,
    cod_rec TEXT DEFAULT NULL,
    num_proc TEXT DEFAULT NULL,
    ind_proc TEXT DEFAULT NULL,
    proc TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL,
    mes_ref TEXT DEFAULT NULL
);

CREATE TABLE reg_e500 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_apur TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL
);

CREATE TABLE reg_e510 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cfop TEXT DEFAULT NULL,
    cst_ipi TEXT DEFAULT NULL,
    vl_cont_ipi decimal(21, 2) DEFAULT NULL,
    vl_bc_ipi decimal(21, 2) DEFAULT NULL,
    vl_ipi decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e520 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    vl_sd_ant_ipi decimal(21, 2) DEFAULT NULL,
    vl_deb_ipi decimal(21, 2) DEFAULT NULL,
    vl_cred_ipi decimal(21, 2) DEFAULT NULL,
    vl_od_ipi decimal(21, 2) DEFAULT NULL,
    vl_oc_ipi decimal(21, 2) DEFAULT NULL,
    vl_sc_ipi decimal(21, 2) DEFAULT NULL,
    vl_sd_ipi decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_e530 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_aj TEXT DEFAULT NULL,
    vl_aj decimal(21, 2) DEFAULT NULL,
    cod_aj TEXT DEFAULT NULL,
    ind_doc TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    descr_aj TEXT DEFAULT NULL
);

CREATE TABLE reg_e990 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_e TEXT DEFAULT NULL
);

CREATE TABLE reg_g001 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE reg_g110 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL,
    saldo_in_icms decimal(21, 2) DEFAULT NULL,
    som_parc decimal(21, 2) DEFAULT NULL,
    vl_trib_exp decimal(21, 2) DEFAULT NULL,
    vl_total decimal(21, 2) DEFAULT NULL,
    ind_per_sai decimal(27, 8) DEFAULT NULL,
    icms_aprop decimal(21, 2) DEFAULT NULL,
    som_icms_oc decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_g125 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_ind_bem TEXT DEFAULT NULL,
    dt_mov date DEFAULT NULL,
    tipo_mov TEXT DEFAULT NULL,
    vl_imob_icms_op decimal(21, 2) DEFAULT NULL,
    vl_imob_icms_st decimal(21, 2) DEFAULT NULL,
    vl_imob_icms_frt decimal(21, 2) DEFAULT NULL,
    vl_imob_icms_dif decimal(21, 2) DEFAULT NULL,
    num_parc TEXT DEFAULT NULL,
    vl_parc_pass decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_g126 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini date DEFAULT NULL,
    dt_fin date DEFAULT NULL,
    num_parc TEXT DEFAULT NULL,
    vl_parc_pass decimal(21, 2) DEFAULT NULL,
    vl_trib_oc decimal(21, 2) DEFAULT NULL,
    vl_total decimal(21, 2) DEFAULT NULL,
    ind_per_sai decimal(27, 8) DEFAULT NULL,
    vl_parc_aprop decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_g130 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_emit TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    cod_mod TEXT DEFAULT NULL,
    serie TEXT DEFAULT NULL,
    num_doc TEXT DEFAULT NULL,
    chv_nfe_cte TEXT DEFAULT NULL,
    dt_doc date DEFAULT NULL
);

CREATE TABLE reg_g140 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    num_item TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL
);

CREATE TABLE reg_g990 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_g TEXT DEFAULT NULL
);

CREATE TABLE reg_h001 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE reg_h005 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_inv date DEFAULT NULL,
    vl_inv decimal(21, 2) DEFAULT NULL,
    mot_inv TEXT DEFAULT NULL
);

CREATE TABLE reg_h010 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    unid TEXT DEFAULT NULL,
    qtd decimal(22, 3) DEFAULT NULL,
    vl_unit decimal(25, 6) DEFAULT NULL,
    vl_item decimal(21, 2) DEFAULT NULL,
    ind_prop TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL,
    txt_compl TEXT DEFAULT NULL,
    cod_cta TEXT DEFAULT NULL,
    vl_item_ir decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_h020 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cst_icms TEXT DEFAULT NULL,
    bl_icms decimal(21, 2) DEFAULT NULL,
    vl_icms decimal(21, 2) DEFAULT NULL
);

CREATE TABLE reg_h990 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_h TEXT DEFAULT NULL
);

CREATE TABLE reg_k001 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    ind_mov TEXT DEFAULT NULL
);

CREATE TABLE reg_k100 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    cod_item date DEFAULT NULL,
    dt_fin date DEFAULT NULL
);

CREATE TABLE reg_k200 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_est date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(17, 3) DEFAULT NULL,
    ind_est TEXT DEFAULT NULL,
    cod_part TEXT DEFAULT NULL
);

CREATE TABLE reg_k220 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_mov date DEFAULT NULL,
    cod_item_ori TEXT DEFAULT NULL,
    cod_item_dest TEXT DEFAULT NULL,
    qtd decimal(17, 3) DEFAULT NULL
);

CREATE TABLE reg_k230 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_ini_op date DEFAULT NULL,
    dt_fin_op date DEFAULT NULL,
    cod_doc_op TEXT DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd_enc decimal(17, 3) DEFAULT NULL
);

CREATE TABLE reg_k235 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_saida date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(17, 3) DEFAULT NULL,
    cod_ins_subst TEXT DEFAULT NULL
);

CREATE TABLE reg_k250 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_prod date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(17, 3) DEFAULT NULL
);

CREATE TABLE reg_k255 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    dt_cons date DEFAULT NULL,
    cod_item TEXT DEFAULT NULL,
    qtd decimal(17, 3) DEFAULT NULL,
    cod_ins_subst TEXT DEFAULT NULL
);

CREATE TABLE reg_k990 (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg TEXT DEFAULT NULL,
    qtd_lin_h TEXT DEFAULT NULL
);