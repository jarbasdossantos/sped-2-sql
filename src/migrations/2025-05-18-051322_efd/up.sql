CREATE TABLE efd_0000
(
    id               INTEGER PRIMARY KEY NOT NULL,
    file_id          INTEGER,
    parent_id        INTEGER DEFAULT NULL,
    reg              TEXT    DEFAULT NULL,
    cod_ver          TEXT    DEFAULT NULL,
    tipo_escrit      TEXT    DEFAULT NULL,
    ind_sit_esp      TEXT    DEFAULT NULL,
    num_rec_anterior TEXT    DEFAULT NULL,
    dt_ini           TEXT    DEFAULT NULL, /* Previously DATE */
    dt_fin           TEXT    DEFAULT NULL, /* Previously DATE */
    nome             TEXT    DEFAULT NULL,
    cnpj             TEXT    DEFAULT NULL,
    uf               TEXT    DEFAULT NULL,
    cod_mun          TEXT    DEFAULT NULL,
    suframa          TEXT    DEFAULT NULL,
    ind_nat_pj       TEXT    DEFAULT NULL,
    ind_ativ         TEXT    DEFAULT NULL
);

CREATE TABLE efd_0001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_0035
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_scp   TEXT    DEFAULT NULL,
    nome_scp  TEXT    DEFAULT NULL,
    inf_comp  TEXT    DEFAULT NULL
);

CREATE TABLE efd_0100
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    nome      TEXT    DEFAULT NULL,
    cpf       TEXT    DEFAULT NULL,
    crc       TEXT    DEFAULT NULL,
    cnpj      TEXT    DEFAULT NULL,
    cep       TEXT    DEFAULT NULL,
    end       TEXT    DEFAULT NULL,
    num       TEXT    DEFAULT NULL,
    compl     TEXT    DEFAULT NULL,
    bairro    TEXT    DEFAULT NULL,
    fone      TEXT    DEFAULT NULL,
    fax       TEXT    DEFAULT NULL,
    email     TEXT    DEFAULT NULL,
    cod_mun   TEXT    DEFAULT NULL
);

CREATE TABLE efd_0110
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER DEFAULT NULL,
    reg           TEXT    DEFAULT NULL,
    cod_inc_trib  TEXT    DEFAULT NULL,
    ind_apro_cred TEXT    DEFAULT NULL,
    cod_tipo_cont TEXT    DEFAULT NULL,
    ind_reg_cum   TEXT    DEFAULT NULL
);

CREATE TABLE efd_0111
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER DEFAULT NULL,
    reg                  TEXT    DEFAULT NULL,
    rec_bru_ncum_trib_mi TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    rec_bru_ncum_nt_mi   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    rec_bru_ncum_exp     TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    rec_bru_cum          TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    rec_bru_total        TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_0120
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    mes_dispensa TEXT    DEFAULT NULL,
    inf_comp     TEXT    DEFAULT NULL
);

CREATE TABLE efd_0140
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_est   TEXT    DEFAULT NULL,
    nome      TEXT    DEFAULT NULL,
    cnpj      TEXT    DEFAULT NULL,
    uf        TEXT    DEFAULT NULL,
    ie        TEXT    DEFAULT NULL,
    cod_mun   TEXT    DEFAULT NULL,
    im        TEXT    DEFAULT NULL,
    suframa   TEXT    DEFAULT NULL
);

CREATE TABLE efd_0145
(
    id                 INTEGER PRIMARY KEY NOT NULL,
    file_id            INTEGER,
    parent_id          INTEGER DEFAULT NULL,
    reg                TEXT    DEFAULT NULL,
    cod_inc_trib       TEXT    DEFAULT NULL,
    vl_rec_tot         TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_rec_ativ        TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_rec_demais_ativ TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    info_compl         TEXT    DEFAULT NULL
);

CREATE TABLE efd_0150
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_part  TEXT    DEFAULT NULL,
    nome      TEXT    DEFAULT NULL,
    cod_pais  TEXT    DEFAULT NULL,
    cnpj      TEXT    DEFAULT NULL,
    cpf       TEXT    DEFAULT NULL,
    ie        TEXT    DEFAULT NULL,
    cod_mun   TEXT    DEFAULT NULL,
    suframa   TEXT    DEFAULT NULL,
    end       TEXT    DEFAULT NULL,
    num       TEXT    DEFAULT NULL,
    compl     TEXT    DEFAULT NULL,
    bairro    TEXT    DEFAULT NULL
);

CREATE TABLE efd_0190
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    unid      TEXT    DEFAULT NULL,
    descr     TEXT    DEFAULT NULL
);

CREATE TABLE efd_0200
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    cod_item     TEXT    DEFAULT NULL,
    descr_item   TEXT    DEFAULT NULL,
    cod_barra    TEXT    DEFAULT NULL,
    cod_ant_item TEXT    DEFAULT NULL,
    unid_inv     TEXT    DEFAULT NULL,
    tipo_item    TEXT    DEFAULT NULL,
    cod_ncm      TEXT    DEFAULT NULL,
    ex_ipi       TEXT    DEFAULT NULL,
    cod_gen      TEXT    DEFAULT NULL,
    cod_lst      TEXT    DEFAULT NULL,
    aliq_icms    TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_0205
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER DEFAULT NULL,
    reg            TEXT    DEFAULT NULL,
    descr_ant_item TEXT    DEFAULT NULL,
    dt_ini         TEXT    DEFAULT NULL, /* Previously DATE */
    dt_fim         TEXT    DEFAULT NULL, /* Previously DATE */
    cod_ant_item   TEXT    DEFAULT NULL
);

CREATE TABLE efd_0206
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_comb  TEXT    DEFAULT NULL
);

CREATE TABLE efd_0208
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_tab   TEXT    DEFAULT NULL,
    cod_gru   TEXT    DEFAULT NULL,
    marca_com TEXT    DEFAULT NULL
);

CREATE TABLE efd_0400
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_nat   TEXT    DEFAULT NULL,
    descr_nat TEXT    DEFAULT NULL
);

CREATE TABLE efd_0450
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_inf   TEXT    DEFAULT NULL,
    txt       TEXT    DEFAULT NULL
);

CREATE TABLE efd_0500
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER DEFAULT NULL,
    reg         TEXT    DEFAULT NULL,
    dt_alt      TEXT    DEFAULT NULL, /* Previously DATE */
    cod_nat_cc  TEXT    DEFAULT NULL,
    ind_cta     TEXT    DEFAULT NULL,
    nivel       TEXT    DEFAULT NULL,
    cod_cta     TEXT    DEFAULT NULL,
    nome_cta    TEXT    DEFAULT NULL,
    cod_cta_ref TEXT    DEFAULT NULL,
    cnpj_est    TEXT    DEFAULT NULL
);

CREATE TABLE efd_0600
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    dt_alt    TEXT    DEFAULT NULL, /* Previously DATE */
    cod_ccus  TEXT    DEFAULT NULL,
    ccus      TEXT    DEFAULT NULL
);

CREATE TABLE efd_0990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_0 TEXT    DEFAULT NULL
);

CREATE TABLE efd_1001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_1010
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    num_proc     TEXT    DEFAULT NULL,
    id_sec_jud   TEXT    DEFAULT NULL,
    id_vara      TEXT    DEFAULT NULL,
    ind_nat_acao TEXT    DEFAULT NULL,
    desc_dec_jud TEXT    DEFAULT NULL,
    dt_sent_jud  TEXT    DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_1020
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    num_proc     TEXT    DEFAULT NULL,
    ind_nat_acao TEXT    DEFAULT NULL,
    dt_dec_adm   TEXT    DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_1100
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER DEFAULT NULL,
    reg                  TEXT    DEFAULT NULL,
    per_apu_cred         TEXT    DEFAULT NULL,
    orig_cred            TEXT    DEFAULT NULL,
    cnpj_suc             TEXT    DEFAULT NULL,
    cod_cred             TEXT    DEFAULT NULL,
    vl_cred_apu          TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_ext_apu      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_tot_cred_apu      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_desc_pa_ant  TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_per_pa_ant   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_dcomp_pa_ant TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sd_cred_disp_efd     TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_desc_efd     TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_per_efd      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_dcomp_efd    TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_trans        TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_out          TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sld_cred_fim         TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_1300
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    ind_nat_ret  TEXT    DEFAULT NULL,
    pr_rec_ret   TEXT    DEFAULT NULL,
    vl_ret_apu   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_ded   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_per   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_dcomp TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sld_ret      TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_1500
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER DEFAULT NULL,
    reg                  TEXT    DEFAULT NULL,
    per_apu_cred         TEXT    DEFAULT NULL,
    orig_cred            TEXT    DEFAULT NULL,
    cnpj_suc             TEXT    DEFAULT NULL,
    cod_cred             TEXT    DEFAULT NULL,
    vl_cred_apu          TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_ext_apu      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_tot_cred_apu      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_desc_pa_ant  TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_per_pa_ant   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_dcomp_pa_ant TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sd_cred_disp_efd     TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_desc_efd     TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_per_efd      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_dcomp_efd    TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_trans        TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_cred_out          TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sld_cred_fim         TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_1700
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER DEFAULT NULL,
    reg          TEXT    DEFAULT NULL,
    ind_nat_ret  TEXT    DEFAULT NULL,
    pr_rec_ret   TEXT    DEFAULT NULL,
    vl_ret_apu   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_ded   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_per   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_ret_dcomp TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    sld_ret      TEXT    DEFAULT NULL /* Previosly NUMERIC */
);

CREATE TABLE efd_1800
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER DEFAULT NULL,
    reg           TEXT    DEFAULT NULL,
    inc_imob      TEXT    DEFAULT NULL,
    rec_receb_ret TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    rec_fin_ret   TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    bc_ret        TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    aliq_ret      TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    vl_rec_uni    TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    dt_rec_uni    TEXT    DEFAULT NULL, /* Previously DATE */
    cod_rec       TEXT    DEFAULT NULL
);

CREATE TABLE efd_1900
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER DEFAULT NULL,
    reg        TEXT    DEFAULT NULL,
    cnpj       TEXT    DEFAULT NULL,
    cod_mod    TEXT    DEFAULT NULL,
    ser        TEXT    DEFAULT NULL,
    sub_ser    TEXT    DEFAULT NULL,
    cod_sit    TEXT    DEFAULT NULL,
    vl_tot_rec TEXT    DEFAULT NULL, /* Previosly NUMERIC */
    quant_doc  TEXT    DEFAULT NULL,
    cst_pis    TEXT    DEFAULT NULL,
    cst_cofins TEXT    DEFAULT NULL,
    cfop       TEXT    DEFAULT NULL,
    info_compl TEXT    DEFAULT NULL,
    cod_cta    TEXT    DEFAULT NULL
);

CREATE TABLE efd_1990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_1 TEXT    DEFAULT NULL
);

CREATE TABLE efd_9001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_9900
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER DEFAULT NULL,
    reg         TEXT    DEFAULT NULL,
    reg_blc     TEXT    DEFAULT NULL,
    qtd_reg_blc VARCHAR DEFAULT NULL
);

CREATE TABLE efd_9990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_9 VARCHAR DEFAULT NULL
);

CREATE TABLE efd_9999
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin   VARCHAR DEFAULT NULL
);

CREATE TABLE efd_a001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_a010
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cnpj      VARCHAR DEFAULT NULL
);

CREATE TABLE efd_a100
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    ind_oper      TEXT           DEFAULT NULL,
    ind_emit      TEXT           DEFAULT NULL,
    cod_part      VARCHAR        DEFAULT NULL,
    cod_sit       VARCHAR        DEFAULT NULL,
    ser           VARCHAR        DEFAULT NULL,
    sub           VARCHAR        DEFAULT NULL,
    num_doc       VARCHAR        DEFAULT NULL,
    chv_nfse      VARCHAR        DEFAULT NULL,
    dt_doc        TEXT           DEFAULT NULL, /* Previously DATE */
    dt_exe_serv   TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc        DECIMAL(21, 2) DEFAULT NULL,
    ind_pgto      TEXT           DEFAULT NULL,
    vl_desc       DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis     DECIMAL(21, 2) DEFAULT NULL,
    vl_pis        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins  DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins     DECIMAL(21, 2) DEFAULT NULL,
    vl_pis_ret    DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins_ret DECIMAL(21, 2) DEFAULT NULL,
    vl_iss        DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_a110
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_inf   VARCHAR DEFAULT NULL,
    txt_compl VARCHAR DEFAULT NULL
);

CREATE TABLE efd_a111
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_a120
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    vl_tot_serv   DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis     DECIMAL(21, 2) DEFAULT NULL,
    vl_pis_imp    DECIMAL(21, 2) DEFAULT NULL,
    dt_pag_pis    TEXT           DEFAULT NULL, /* Previously DATE */
    vl_bc_cofins  DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins_imp DECIMAL(21, 2) DEFAULT NULL,
    dt_pag_cofins TEXT           DEFAULT NULL, /* Previously DATE */
    loc_exe_serv  TEXT           DEFAULT NULL
);

CREATE TABLE efd_a170
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    num_item      TEXT           DEFAULT NULL,
    cod_item      VARCHAR        DEFAULT NULL,
    descr_compl   VARCHAR        DEFAULT NULL,
    vl_item       DECIMAL(21, 2) DEFAULT NULL,
    vl_desc       DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred   VARCHAR        DEFAULT NULL,
    ind_orig_cred TEXT           DEFAULT NULL,
    cst_pis       VARCHAR        DEFAULT NULL,
    vl_bc_pis     DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis      DECIMAL(21, 2) DEFAULT NULL,
    vl_pis        DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins    VARCHAR        DEFAULT NULL,
    vl_bc_cofins  DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins   DECIMAL(8, 2)  DEFAULT NULL,
    vl_cofins     DECIMAL(21, 2) DEFAULT NULL,
    cod_cta       VARCHAR        DEFAULT NULL,
    cod_ccus      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_a990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_a VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_c010
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cnpj      VARCHAR DEFAULT NULL,
    ind_escri TEXT    DEFAULT NULL
);

CREATE TABLE efd_c100
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    ind_oper      TEXT           DEFAULT NULL,
    ind_emit      TEXT           DEFAULT NULL,
    cod_part      VARCHAR        DEFAULT NULL,
    cod_mod       VARCHAR        DEFAULT NULL,
    cod_sit       VARCHAR        DEFAULT NULL,
    ser           VARCHAR        DEFAULT NULL,
    num_doc       VARCHAR        DEFAULT NULL,
    chv_nfe       VARCHAR        DEFAULT NULL,
    dt_doc        TEXT           DEFAULT NULL, /* Previously DATE */
    dt_e_s        TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc        DECIMAL(21, 2) DEFAULT NULL,
    ind_pgto      TEXT           DEFAULT NULL,
    vl_desc       DECIMAL(21, 2) DEFAULT NULL,
    vl_abat_nt    DECIMAL(21, 2) DEFAULT NULL,
    vl_merc       DECIMAL(21, 2) DEFAULT NULL,
    ind_frt       TEXT           DEFAULT NULL,
    vl_frt        DECIMAL(21, 2) DEFAULT NULL,
    vl_seg        DECIMAL(21, 2) DEFAULT NULL,
    vl_out_da     DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms    DECIMAL(21, 2) DEFAULT NULL,
    vl_icms       DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms_st DECIMAL(21, 2) DEFAULT NULL,
    vl_icms_st    DECIMAL(21, 2) DEFAULT NULL,
    vl_ipi        DECIMAL(21, 2) DEFAULT NULL,
    vl_pis        DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins     DECIMAL(21, 2) DEFAULT NULL,
    vl_pis_st     DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins_st  DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c110
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_inf   VARCHAR DEFAULT NULL,
    txt_compl VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c111
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c120
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    cod_doc_imp   TEXT           DEFAULT NULL,
    num_doc_imp   VARCHAR        DEFAULT NULL,
    vl_pis_imp    DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins_imp DECIMAL(21, 2) DEFAULT NULL,
    num_acdraw    VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c170
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    num_item          VARCHAR        DEFAULT NULL,
    cod_item          VARCHAR        DEFAULT NULL,
    descr_compl       VARCHAR        DEFAULT NULL,
    qtd               DECIMAL(24, 5) DEFAULT NULL,
    unid              VARCHAR        DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_desc           DECIMAL(21, 2) DEFAULT NULL,
    ind_mov           TEXT           DEFAULT NULL,
    cst_icms          VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    cod_nat           VARCHAR        DEFAULT NULL,
    vl_bc_icms        DECIMAL(21, 2) DEFAULT NULL,
    aliq_icms         DECIMAL(8, 2)  DEFAULT NULL,
    vl_icms           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms_st     DECIMAL(21, 2) DEFAULT NULL,
    aliq_st           DECIMAL(8, 2)  DEFAULT NULL,
    vl_icms_st        DECIMAL(21, 2) DEFAULT NULL,
    ind_apur          TEXT           DEFAULT NULL,
    cst_ipi           VARCHAR        DEFAULT NULL,
    cod_enq           VARCHAR        DEFAULT NULL,
    vl_bc_ipi         DECIMAL(21, 2) DEFAULT NULL,
    aliq_ipi          DECIMAL(8, 2)  DEFAULT NULL,
    vl_ipi            DECIMAL(21, 2) DEFAULT NULL,
    cst_pis           VARCHAR        DEFAULT NULL,
    vl_bc_pis         DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis          DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis      DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant    DECIMAL(23, 4) DEFAULT NULL,
    vl_pis            DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c180
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cod_mod     VARCHAR        DEFAULT NULL,
    dt_doc_ini  TEXT           DEFAULT NULL, /* Previously DATE */
    dt_doc_fin  TEXT           DEFAULT NULL, /* Previously DATE */
    cod_item    VARCHAR        DEFAULT NULL,
    cod_ncm     VARCHAR        DEFAULT NULL,
    ex_ipi      VARCHAR        DEFAULT NULL,
    vl_tot_item DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c181
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    cfop           TEXT           DEFAULT NULL,
    vl_item        DECIMAL(21, 2) DEFAULT NULL,
    vl_desc        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c185
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_desc           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c188
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c190
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cod_mod     VARCHAR        DEFAULT NULL,
    dt_ref_ini  TEXT           DEFAULT NULL, /* Previously DATE */
    dt_ref_fin  TEXT           DEFAULT NULL, /* Previously DATE */
    cod_item    VARCHAR        DEFAULT NULL,
    cod_ncm     VARCHAR        DEFAULT NULL,
    ex_ipi      VARCHAR        DEFAULT NULL,
    vl_tot_item DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c191
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cnpj_cpf_part  VARCHAR        DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    cfop           TEXT           DEFAULT NULL,
    vl_item        DECIMAL(21, 2) DEFAULT NULL,
    vl_desc        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c195
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cnpj_cpf_part     VARCHAR        DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_desc           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c198
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c199
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    cod_doc_imp   TEXT           DEFAULT NULL,
    num_doc_imp   VARCHAR        DEFAULT NULL,
    vl_pis_imp    DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins_imp DECIMAL(21, 2) DEFAULT NULL,
    num_acdraw    VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c380
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cod_mod     VARCHAR        DEFAULT NULL,
    dt_doc_ini  TEXT           DEFAULT NULL, /* Previously DATE */
    dt_doc_fin  TEXT           DEFAULT NULL, /* Previously DATE */
    num_doc_ini VARCHAR        DEFAULT NULL,
    num_doc_fin VARCHAR        DEFAULT NULL,
    vl_doc      DECIMAL(21, 2) DEFAULT NULL,
    vl_doc_canc DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c381
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    cod_item       VARCHAR        DEFAULT NULL,
    vl_item        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c385
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    cod_item          VARCHAR        DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c395
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cod_mod   VARCHAR        DEFAULT NULL,
    cod_part  VARCHAR        DEFAULT NULL,
    ser       VARCHAR        DEFAULT NULL,
    sub_ser   VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    dt_doc    TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc    DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c396
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cod_item     VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    vl_desc      DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred  VARCHAR        DEFAULT NULL,
    cst_pis      VARCHAR        DEFAULT NULL,
    vl_bc_pis    DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis     DECIMAL(12, 4) DEFAULT NULL,
    vl_pis       DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c400
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cod_mod   VARCHAR DEFAULT NULL,
    ecf_mod   VARCHAR DEFAULT NULL,
    ecf_fab   VARCHAR DEFAULT NULL,
    ecf_cx    VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c405
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    dt_doc      TEXT           DEFAULT NULL, /* Previously DATE */
    cro         VARCHAR        DEFAULT NULL,
    crz         VARCHAR        DEFAULT NULL,
    num_coo_fin VARCHAR        DEFAULT NULL,
    gt_fin      DECIMAL(21, 2) DEFAULT NULL,
    vl_brt      DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c481
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    vl_item        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cod_item       VARCHAR        DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c485
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_item          VARCHAR        DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c489
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c490
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER DEFAULT NULL,
    reg        TEXT    DEFAULT NULL,
    dt_doc_ini TEXT    DEFAULT NULL, /* Previously DATE */
    dt_doc_fin TEXT    DEFAULT NULL, /* Previously DATE */
    cod_mod    VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c491
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cod_item       VARCHAR        DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    cfop           TEXT           DEFAULT NULL,
    vl_item        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c495
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cod_item          VARCHAR        DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c499
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c500
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cod_part  VARCHAR        DEFAULT NULL,
    cod_mod   VARCHAR        DEFAULT NULL,
    cod_sit   VARCHAR        DEFAULT NULL,
    ser       TEXT           DEFAULT NULL,
    sub       VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    dt_doc    TEXT           DEFAULT NULL, /* Previously DATE */
    dt_e_s    TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc    DECIMAL(21, 2) DEFAULT NULL,
    vl_icms   DECIMAL(21, 2) DEFAULT NULL,
    cod_inf   VARCHAR        DEFAULT NULL,
    vl_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c501
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cst_pis     VARCHAR        DEFAULT NULL,
    vl_item     DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred VARCHAR        DEFAULT NULL,
    vl_bc_pis   DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis    DECIMAL(12, 4) DEFAULT NULL,
    vl_pis      DECIMAL(21, 2) DEFAULT NULL,
    cod_cta     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c505
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred  VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c509
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c600
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    cod_mod       VARCHAR        DEFAULT NULL,
    cod_mun       VARCHAR        DEFAULT NULL,
    ser           TEXT           DEFAULT NULL,
    sub           VARCHAR        DEFAULT NULL,
    cod_cons      VARCHAR        DEFAULT NULL,
    qtd_cons      VARCHAR        DEFAULT NULL,
    qtd_canc      VARCHAR        DEFAULT NULL,
    dt_doc        TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc        DECIMAL(21, 2) DEFAULT NULL,
    vl_desc       DECIMAL(21, 2) DEFAULT NULL,
    cons          VARCHAR        DEFAULT NULL,
    vl_forn       DECIMAL(21, 2) DEFAULT NULL,
    vl_serv_nt    DECIMAL(21, 2) DEFAULT NULL,
    vl_terc       DECIMAL(21, 2) DEFAULT NULL,
    vl_da         DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms    DECIMAL(21, 2) DEFAULT NULL,
    vl_icms       DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms_st DECIMAL(21, 2) DEFAULT NULL,
    vl_icms_st    DECIMAL(21, 2) DEFAULT NULL,
    vl_pis        DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins     DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_c601
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cst_pis   VARCHAR        DEFAULT NULL,
    vl_item   DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis  DECIMAL(12, 4) DEFAULT NULL,
    vl_pis    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta   VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c605
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c609
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_c860
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       VARCHAR DEFAULT NULL,
    cod_mod   VARCHAR DEFAULT NULL,
    nr_sat    VARCHAR DEFAULT NULL,
    dt_doc    VARCHAR DEFAULT NULL, /* Previously DATE */
    doc_ini   VARCHAR DEFAULT NULL,
    doc_fim   VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c870
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER             NOT NULL,
    reg          VARCHAR        DEFAULT NULL,
    cod_item     VARCHAR        DEFAULT NULL,
    cfop         VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    vl_desc      DECIMAL(21, 2) DEFAULT NULL,
    cst_pis      VARCHAR        DEFAULT NULL,
    vl_bc_pis    DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis     DECIMAL(12, 4) DEFAULT NULL,
    vl_pis       DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c880
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER             NOT NULL,
    reg               VARCHAR        DEFAULT NULL,
    cod_item          VARCHAR        DEFAULT NULL,
    cfop              VARCHAR        DEFAULT NULL,
    vl_item           DECIMAL(21, 2) DEFAULT NULL,
    vl_desc           DECIMAL(21, 2) DEFAULT NULL,
    cst_pis           VARCHAR        DEFAULT NULL,
    quant_bc_pis      DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant    DECIMAL(23, 4) DEFAULT NULL,
    vl_pis            DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_c890
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       VARCHAR DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  VARCHAR DEFAULT NULL
);

CREATE TABLE efd_c990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_c VARCHAR DEFAULT NULL
);

CREATE TABLE efd_d001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_d010
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cnpj      VARCHAR DEFAULT NULL
);

CREATE TABLE efd_d100
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    ind_oper    TEXT           DEFAULT NULL,
    ind_emit    TEXT           DEFAULT NULL,
    cod_part    VARCHAR        DEFAULT NULL,
    cod_mod     VARCHAR        DEFAULT NULL,
    cod_sit     VARCHAR        DEFAULT NULL,
    ser         TEXT           DEFAULT NULL,
    sub         VARCHAR        DEFAULT NULL,
    num_doc     VARCHAR        DEFAULT NULL,
    chv_cte     VARCHAR        DEFAULT NULL,
    dt_doc      TEXT           DEFAULT NULL, /* Previously DATE */
    dt_a_p      TEXT           DEFAULT NULL, /* Previously DATE */
    tp_cte      TEXT           DEFAULT NULL,
    chv_cte_ref VARCHAR        DEFAULT NULL,
    vl_doc      DECIMAL(21, 2) DEFAULT NULL,
    vl_desc     DECIMAL(21, 2) DEFAULT NULL,
    ind_frt     TEXT           DEFAULT NULL,
    vl_serv     DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms  DECIMAL(21, 2) DEFAULT NULL,
    vl_icms     DECIMAL(21, 2) DEFAULT NULL,
    vl_nt       DECIMAL(21, 2) DEFAULT NULL,
    cod_inf     VARCHAR        DEFAULT NULL,
    cod_cta     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d101
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    ind_nat_frt TEXT           DEFAULT NULL,
    vl_item     DECIMAL(21, 2) DEFAULT NULL,
    cst_pis     VARCHAR        DEFAULT NULL,
    nat_bc_cred VARCHAR        DEFAULT NULL,
    vl_bc_pis   DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis    DECIMAL(12, 4) DEFAULT NULL,
    vl_pis      DECIMAL(21, 2) DEFAULT NULL,
    cod_cta     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d105
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    ind_nat_frt  TEXT           DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    nat_bc_cred  VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d111
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d200
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cod_mod     VARCHAR        DEFAULT NULL,
    cod_sit     VARCHAR        DEFAULT NULL,
    ser         TEXT           DEFAULT NULL,
    sub         VARCHAR        DEFAULT NULL,
    num_doc_ini VARCHAR        DEFAULT NULL,
    num_doc_fin VARCHAR        DEFAULT NULL,
    cfop        TEXT           DEFAULT NULL,
    dt_ref      TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc      DECIMAL(21, 2) DEFAULT NULL,
    vl_desc     DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_d201
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cst_pis   VARCHAR        DEFAULT NULL,
    vl_item   DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis  DECIMAL(12, 4) DEFAULT NULL,
    vl_pis    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta   VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d205
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d209
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d300
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cod_mod      VARCHAR        DEFAULT NULL,
    ser          TEXT           DEFAULT NULL,
    sub          VARCHAR        DEFAULT NULL,
    num_doc_ini  VARCHAR        DEFAULT NULL,
    num_doc_fin  VARCHAR        DEFAULT NULL,
    cfop         TEXT           DEFAULT NULL,
    dt_ref       TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc       DECIMAL(21, 2) DEFAULT NULL,
    vl_desc      DECIMAL(21, 2) DEFAULT NULL,
    cst_pis      VARCHAR        DEFAULT NULL,
    vl_bc_pis    DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis     DECIMAL(12, 4) DEFAULT NULL,
    vl_pis       DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d309
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d350
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cod_mod           VARCHAR        DEFAULT NULL,
    ecf_mod           VARCHAR        DEFAULT NULL,
    ecf_fab           VARCHAR        DEFAULT NULL,
    dt_doc            TEXT           DEFAULT NULL, /* Previously DATE */
    cro               VARCHAR        DEFAULT NULL,
    crz               VARCHAR        DEFAULT NULL,
    num_coo_fin       VARCHAR        DEFAULT NULL,
    gt_fin            DECIMAL(21, 2) DEFAULT NULL,
    vl_brt            DECIMAL(21, 2) DEFAULT NULL,
    cst_pis           VARCHAR        DEFAULT NULL,
    vl_bc_pis         DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis          DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis      DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant    DECIMAL(23, 4) DEFAULT NULL,
    vl_pis            DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    vl_bc_cofins      DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d359
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d500
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    ind_oper   TEXT           DEFAULT NULL,
    ind_emit   TEXT           DEFAULT NULL,
    cod_part   VARCHAR        DEFAULT NULL,
    cod_mod    VARCHAR        DEFAULT NULL,
    cod_sit    VARCHAR        DEFAULT NULL,
    ser        TEXT           DEFAULT NULL,
    sub        VARCHAR        DEFAULT NULL,
    num_doc    VARCHAR        DEFAULT NULL,
    dt_doc     TEXT           DEFAULT NULL, /* Previously DATE */
    dt_a_p     TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc     DECIMAL(21, 2) DEFAULT NULL,
    vl_desc    DECIMAL(21, 2) DEFAULT NULL,
    vl_serv    DECIMAL(21, 2) DEFAULT NULL,
    vl_serv_nt DECIMAL(21, 2) DEFAULT NULL,
    vl_terc    DECIMAL(21, 2) DEFAULT NULL,
    vl_da      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms DECIMAL(21, 2) DEFAULT NULL,
    vl_icms    DECIMAL(21, 2) DEFAULT NULL,
    cod_inf    VARCHAR        DEFAULT NULL,
    vl_pis     DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins  DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_d501
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER,
    parent_id   INTEGER        DEFAULT NULL,
    reg         TEXT           DEFAULT NULL,
    cst_pis     VARCHAR        DEFAULT NULL,
    vl_item     DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred VARCHAR        DEFAULT NULL,
    vl_bc_pis   DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis    DECIMAL(12, 4) DEFAULT NULL,
    vl_pis      DECIMAL(21, 2) DEFAULT NULL,
    cod_cta     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d505
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred  VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d509
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d600
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    cod_mod    VARCHAR        DEFAULT NULL,
    cod_mun    VARCHAR        DEFAULT NULL,
    ser        TEXT           DEFAULT NULL,
    sub        VARCHAR        DEFAULT NULL,
    ind_rec    TEXT           DEFAULT NULL,
    qtd_cons   VARCHAR        DEFAULT NULL,
    dt_doc_ini TEXT           DEFAULT NULL, /* Previously DATE */
    dt_doc_fin TEXT           DEFAULT NULL, /* Previously DATE */
    vl_doc     DECIMAL(21, 2) DEFAULT NULL,
    vl_desc    DECIMAL(21, 2) DEFAULT NULL,
    vl_serv    DECIMAL(21, 2) DEFAULT NULL,
    vl_serv_nt DECIMAL(21, 2) DEFAULT NULL,
    vl_terc    DECIMAL(21, 2) DEFAULT NULL,
    vl_da      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_icms DECIMAL(21, 2) DEFAULT NULL,
    vl_icms    DECIMAL(21, 2) DEFAULT NULL,
    vl_pis     DECIMAL(21, 2) DEFAULT NULL,
    vl_cofins  DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_d601
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cod_class TEXT           DEFAULT NULL,
    vl_item   DECIMAL(21, 2) DEFAULT NULL,
    vl_desc   DECIMAL(21, 2) DEFAULT NULL,
    cst_pis   VARCHAR        DEFAULT NULL,
    vl_bc_pis DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis  DECIMAL(12, 4) DEFAULT NULL,
    vl_pis    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta   VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d605
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cod_class    TEXT           DEFAULT NULL,
    vl_item      DECIMAL(21, 2) DEFAULT NULL,
    vl_desc      DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins   VARCHAR        DEFAULT NULL,
    vl_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins  DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_d609
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_d990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_d VARCHAR DEFAULT NULL
);

CREATE TABLE efd_f001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_f010
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cnpj      VARCHAR DEFAULT NULL
);

CREATE TABLE efd_f100
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    ind_oper      TEXT           DEFAULT NULL,
    cod_part      VARCHAR        DEFAULT NULL,
    cod_item      VARCHAR        DEFAULT NULL,
    dt_oper       TEXT           DEFAULT NULL, /* Previously DATE */
    vl_oper       DECIMAL(21, 2) DEFAULT NULL,
    cst_pis       VARCHAR        DEFAULT NULL,
    vl_bc_pis     DECIMAL(23, 4) DEFAULT NULL,
    aliq_pis      DECIMAL(12, 4) DEFAULT NULL,
    vl_pis        DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins    VARCHAR        DEFAULT NULL,
    vl_bc_cofins  DECIMAL(23, 4) DEFAULT NULL,
    aliq_cofins   DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins     DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred   VARCHAR        DEFAULT NULL,
    ind_orig_cred TEXT           DEFAULT NULL,
    cod_cta       VARCHAR        DEFAULT NULL,
    cod_ccus      VARCHAR        DEFAULT NULL,
    desc_doc_oper VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f111
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f120
(
    id                    INTEGER PRIMARY KEY NOT NULL,
    file_id               INTEGER,
    parent_id             INTEGER        DEFAULT NULL,
    reg                   TEXT           DEFAULT NULL,
    nat_bc_cred           VARCHAR        DEFAULT NULL,
    ident_bem_imob        VARCHAR        DEFAULT NULL,
    ind_orig_cred         TEXT           DEFAULT NULL,
    ind_util_bem_imob     TEXT           DEFAULT NULL,
    vl_oper_dep           DECIMAL(21, 2) DEFAULT NULL,
    parc_oper_nao_bc_cred DECIMAL(21, 2) DEFAULT NULL,
    cst_pis               VARCHAR        DEFAULT NULL,
    vl_bc_pis             DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis              DECIMAL(12, 4) DEFAULT NULL,
    vl_pis                DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins            VARCHAR        DEFAULT NULL,
    vl_bc_cofins          DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins           DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins             DECIMAL(21, 2) DEFAULT NULL,
    cod_cta               VARCHAR        DEFAULT NULL,
    cod_ccus              VARCHAR        DEFAULT NULL,
    desc_bem_imob         VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f129
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f130
(
    id                    INTEGER PRIMARY KEY NOT NULL,
    file_id               INTEGER,
    parent_id             INTEGER        DEFAULT NULL,
    reg                   TEXT           DEFAULT NULL,
    nat_bc_cred           VARCHAR        DEFAULT NULL,
    ident_bem_imob        VARCHAR        DEFAULT NULL,
    ind_orig_cred         TEXT           DEFAULT NULL,
    ind_util_bem_imob     TEXT           DEFAULT NULL,
    mes_oper_aquis        VARCHAR        DEFAULT NULL,
    vl_oper_aquis         DECIMAL(21, 2) DEFAULT NULL,
    parc_oper_nao_bc_cred DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cred            DECIMAL(21, 2) DEFAULT NULL,
    ind_nr_parc           TEXT           DEFAULT NULL,
    cst_pis               VARCHAR        DEFAULT NULL,
    vl_bc_pis             DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis              DECIMAL(12, 4) DEFAULT NULL,
    vl_pis                DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins            VARCHAR        DEFAULT NULL,
    vl_bc_cofins          DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins           DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins             DECIMAL(21, 2) DEFAULT NULL,
    cod_cta               VARCHAR        DEFAULT NULL,
    cod_ccus              VARCHAR        DEFAULT NULL,
    desc_bem_imob         VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f139
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f150
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    nat_bc_cred    VARCHAR        DEFAULT NULL,
    vl_tot_est     DECIMAL(21, 2) DEFAULT NULL,
    est_imp        DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_est      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_men_est  DECIMAL(21, 2) DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_pis    DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins     VARCHAR        DEFAULT NULL,
    aliq_cofins    DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_cofins DECIMAL(21, 2) DEFAULT NULL,
    desc_est       VARCHAR        DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f200
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    ind_oper       VARCHAR        DEFAULT NULL,
    unid_imob      VARCHAR        DEFAULT NULL,
    ident_emp      VARCHAR        DEFAULT NULL,
    desc_unid_imob VARCHAR        DEFAULT NULL,
    num_cont       VARCHAR        DEFAULT NULL,
    cpf_cnpj_adqu  VARCHAR        DEFAULT NULL,
    dt_oper        TEXT           DEFAULT NULL, /* Previously DATE */
    vl_tot_vend    DECIMAL(21, 2) DEFAULT NULL,
    vl_rec_acum    DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_rec     DECIMAL(21, 2) DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins     VARCHAR        DEFAULT NULL,
    vl_bc_cofins   DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins    DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins      DECIMAL(21, 2) DEFAULT NULL,
    perc_rec_receb DECIMAL(8, 2)  DEFAULT NULL,
    ind_nat_emp    TEXT           DEFAULT NULL,
    inf_comp       VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f205
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    file_id                 INTEGER,
    parent_id               INTEGER        DEFAULT NULL,
    reg                     TEXT           DEFAULT NULL,
    vl_cus_inc_acum_ant     DECIMAL(21, 2) DEFAULT NULL,
    vl_cus_inc_per_esc      DECIMAL(21, 2) DEFAULT NULL,
    vl_cus_inc_acum         DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_bc_cus_inc_acum  DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cus_inc           DECIMAL(21, 2) DEFAULT NULL,
    cst_pis                 VARCHAR        DEFAULT NULL,
    aliq_pis                DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_pis_acum        DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_pis_desc_ant    DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_pis_desc        DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_pis_desc_fut    DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins              VARCHAR        DEFAULT NULL,
    aliq_cofins             DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_cofins_acum     DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc_ant DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc     DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_cofins_desc_fut DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_f210
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    file_id             INTEGER,
    parent_id           INTEGER        DEFAULT NULL,
    reg                 TEXT           DEFAULT NULL,
    vl_cus_orc          DECIMAL(21, 2) DEFAULT NULL,
    vl_exc              DECIMAL(21, 2) DEFAULT NULL,
    vl_cus_orc_aju      DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cred          DECIMAL(21, 2) DEFAULT NULL,
    cst_pis             VARCHAR        DEFAULT NULL,
    aliq_pis            DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_pis_util    DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins          VARCHAR        DEFAULT NULL,
    aliq_cofins         DECIMAL(12, 4) DEFAULT NULL,
    vl_cred_cofins_util DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_f211
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f500
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    vl_rec_caixa   DECIMAL(21, 2) DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    vl_desc_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins     VARCHAR        DEFAULT NULL,
    vl_desc_cofins DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins   DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins    DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins      DECIMAL(21, 2) DEFAULT NULL,
    cod_mod        VARCHAR        DEFAULT NULL,
    cfop           TEXT           DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL,
    info_compl     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f509
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f510
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    vl_rec_caixa      DECIMAL(21, 2) DEFAULT NULL,
    cst_pis           VARCHAR        DEFAULT NULL,
    vl_desc_pis       DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_pis      DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant    DECIMAL(23, 4) DEFAULT NULL,
    vl_pis            DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    vl_desc_cofins    DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_mod           VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL,
    info_compl        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f519
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f525
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    vl_rec     DECIMAL(21, 2) DEFAULT NULL,
    ind_rec    VARCHAR        DEFAULT NULL,
    cnpj_cpf   VARCHAR        DEFAULT NULL,
    num_doc    VARCHAR        DEFAULT NULL,
    cod_item   VARCHAR        DEFAULT NULL,
    vl_rec_det DECIMAL(21, 2) DEFAULT NULL,
    cst_pis    VARCHAR        DEFAULT NULL,
    cst_cofins VARCHAR        DEFAULT NULL,
    info_compl VARCHAR        DEFAULT NULL,
    cod_cta    VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f550
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    vl_rec_comp    DECIMAL(21, 2) DEFAULT NULL,
    cst_pis        VARCHAR        DEFAULT NULL,
    vl_desc_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins     VARCHAR        DEFAULT NULL,
    vl_desc_cofins DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins   DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins    DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins      DECIMAL(21, 2) DEFAULT NULL,
    cod_mod        VARCHAR        DEFAULT NULL,
    cfop           TEXT           DEFAULT NULL,
    cod_cta        VARCHAR        DEFAULT NULL,
    info_compl     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f559
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f560
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    vl_rec_comp       DECIMAL(21, 2) DEFAULT NULL,
    cst_pis           VARCHAR        DEFAULT NULL,
    vl_desc_pis       DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_pis      DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant    DECIMAL(12, 4) DEFAULT NULL,
    vl_pis            DECIMAL(21, 2) DEFAULT NULL,
    cst_cofins        VARCHAR        DEFAULT NULL,
    vl_desc_cofins    DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(12, 4) DEFAULT NULL,
    vl_cofins         DECIMAL(21, 2) DEFAULT NULL,
    cod_mod           VARCHAR        DEFAULT NULL,
    cfop              TEXT           DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL,
    info_compl        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f569
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_f600
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    ind_nat_ret   VARCHAR        DEFAULT NULL,
    dt_ret        TEXT           DEFAULT NULL, /* Previously DATE */
    vl_bc_ret     DECIMAL(23, 4) DEFAULT NULL,
    vl_ret        DECIMAL(21, 2) DEFAULT NULL,
    cod_rec       TEXT           DEFAULT NULL,
    ind_nat_rec   TEXT           DEFAULT NULL,
    cnpj          VARCHAR        DEFAULT NULL,
    vl_ret_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_ret_cofins DECIMAL(21, 2) DEFAULT NULL,
    ind_dec       TEXT           DEFAULT NULL
);

CREATE TABLE efd_f700
(
    id            INTEGER PRIMARY KEY NOT NULL,
    file_id       INTEGER,
    parent_id     INTEGER        DEFAULT NULL,
    reg           TEXT           DEFAULT NULL,
    ind_ori_ded   VARCHAR        DEFAULT NULL,
    ind_nat_ded   TEXT           DEFAULT NULL,
    vl_ded_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_ded_cofins DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_oper    DECIMAL(21, 2) DEFAULT NULL,
    cnpj          VARCHAR        DEFAULT NULL,
    inf_comp      VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_f800
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    ind_nat_even   VARCHAR        DEFAULT NULL,
    dt_even        TEXT           DEFAULT NULL, /* Previously DATE */
    cnpj_suced     VARCHAR        DEFAULT NULL,
    pa_cont_cred   VARCHAR        DEFAULT NULL,
    cod_cred       VARCHAR        DEFAULT NULL,
    vl_cred_pis    DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_cofins DECIMAL(21, 2) DEFAULT NULL,
    per_cred_cis   DECIMAL(8, 2)  DEFAULT NULL
);

CREATE TABLE efd_f990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_f VARCHAR DEFAULT NULL
);

CREATE TABLE efd_i001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_i010
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER DEFAULT NULL,
    reg        TEXT    DEFAULT NULL,
    cnpj       VARCHAR DEFAULT NULL,
    ind_ativ   VARCHAR DEFAULT NULL,
    info_compl VARCHAR DEFAULT NULL
);

CREATE TABLE efd_i100
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    vl_rec_fin     DECIMAL(21, 2) DEFAULT NULL,
    cst            VARCHAR        DEFAULT NULL,
    vl_tot_ded_ger DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_ded_esp DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis      DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(10, 2) DEFAULT NULL,
    vl_pis         DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins   DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins    DECIMAL(10, 2) DEFAULT NULL,
    vl_cofins      DECIMAL(21, 2) DEFAULT NULL,
    inf_comp       VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_i199
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_i200
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    num_campo VARCHAR        DEFAULT NULL,
    cod_det   VARCHAR        DEFAULT NULL,
    vl_det    DECIMAL(21, 2) DEFAULT NULL,
    cod_cta   VARCHAR        DEFAULT NULL,
    inf_comp  VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_i299
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_i300
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    cod_comp  VARCHAR        DEFAULT NULL,
    vl_comp   DECIMAL(21, 2) DEFAULT NULL,
    cod_cta   VARCHAR        DEFAULT NULL,
    inf_comp  VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_i399
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_i990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_i VARCHAR DEFAULT NULL
);

CREATE TABLE efd_m001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_m100
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER,
    parent_id      INTEGER        DEFAULT NULL,
    reg            TEXT           DEFAULT NULL,
    cod_cred       VARCHAR        DEFAULT NULL,
    ind_cred_ori   TEXT           DEFAULT NULL,
    vl_bc_cred     DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_pis   DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cred        DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_acres  DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc  DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_dif    DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_disp   DECIMAL(21, 2) DEFAULT NULL,
    ind_desc_cred  TEXT           DEFAULT NULL,
    vl_cred_desc   DECIMAL(21, 2) DEFAULT NULL,
    sld_cred       DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m105
(
    id               INTEGER PRIMARY KEY NOT NULL,
    file_id          INTEGER,
    parent_id        INTEGER        DEFAULT NULL,
    reg              TEXT           DEFAULT NULL,
    nat_bc_cred      VARCHAR        DEFAULT NULL,
    cst_pis          VARCHAR        DEFAULT NULL,
    vl_bc_pis_tot    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis_cum    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis_nc     DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_pis        DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_pis_tot DECIMAL(22, 3) DEFAULT NULL,
    quant_bc_pis     DECIMAL(22, 3) DEFAULT NULL,
    desc_cred        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m110
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    ind_aj    TEXT           DEFAULT NULL,
    vl_aj     DECIMAL(21, 2) DEFAULT NULL,
    cod_aj    VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    descr_aj  VARCHAR        DEFAULT NULL,
    dt_ref    TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m200
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER        DEFAULT NULL,
    reg                  TEXT           DEFAULT NULL,
    vl_tot_cont_nc_per   DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cred_desc     DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cred_desc_ant DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_nc_dev   DECIMAL(21, 2) DEFAULT NULL,
    vl_ret_nc            DECIMAL(21, 2) DEFAULT NULL,
    vl_out_ded_nc        DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_nc_rec       DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_cum_per  DECIMAL(21, 2) DEFAULT NULL,
    vl_ret_cum           DECIMAL(21, 2) DEFAULT NULL,
    vl_out_ded_cum       DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_cum_rec      DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_rec      DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m205
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    num_campo VARCHAR        DEFAULT NULL,
    cod_rec   VARCHAR        DEFAULT NULL,
    vl_debito DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m210
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER        DEFAULT NULL,
    reg                  TEXT           DEFAULT NULL,
    cod_cont             VARCHAR        DEFAULT NULL,
    vl_rec_brt           DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont           DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis             DECIMAL(12, 4) DEFAULT NULL,
    vl_ajus_acres_bc_pis DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc_bc_pis DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont_ajus      DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_pis         DECIMAL(22, 3) DEFAULT NULL,
    aliq_pis_quant       DECIMAL(23, 4) DEFAULT NULL,
    vl_cont_apur         DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_acres        DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc        DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer        DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer_ant    DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_per          DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m211
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    file_id                 INTEGER,
    parent_id               INTEGER        DEFAULT NULL,
    reg                     TEXT           DEFAULT NULL,
    ind_tip_coop            VARCHAR        DEFAULT NULL,
    vl_bc_cont_ant_exc_coop DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_coop_ger         DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_esp_coop         DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont              DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m220
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    ind_aj    TEXT           DEFAULT NULL,
    vl_aj     DECIMAL(21, 2) DEFAULT NULL,
    cod_aj    VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    descr_aj  VARCHAR        DEFAULT NULL,
    dt_ref    TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m230
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cnpj         VARCHAR        DEFAULT NULL,
    vl_vend      DECIMAL(21, 2) DEFAULT NULL,
    vl_nao_receb DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_dif  DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_dif  DECIMAL(21, 2) DEFAULT NULL,
    cod_cred     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m300
(
    id                 INTEGER PRIMARY KEY NOT NULL,
    file_id            INTEGER,
    parent_id          INTEGER        DEFAULT NULL,
    reg                TEXT           DEFAULT NULL,
    cod_cont           VARCHAR        DEFAULT NULL,
    vl_cont_apur_difer DECIMAL(21, 2) DEFAULT NULL,
    nat_cred_desc      VARCHAR        DEFAULT NULL,
    vl_cred_desc_difer DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer_ant  DECIMAL(21, 2) DEFAULT NULL,
    per_apur           VARCHAR        DEFAULT NULL,
    dt_receb           TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m350
(
    id              INTEGER PRIMARY KEY NOT NULL,
    file_id         INTEGER,
    parent_id       INTEGER        DEFAULT NULL,
    reg             TEXT           DEFAULT NULL,
    vl_tot_fol      DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_bc       DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_bc       DECIMAL(21, 2) DEFAULT NULL,
    aliq_pis_fol    DECIMAL(8, 2)  DEFAULT NULL,
    vl_tot_cont_fol DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m400
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    cst_pis    VARCHAR        DEFAULT NULL,
    vl_tot_rec DECIMAL(21, 2) DEFAULT NULL,
    cod_cta    VARCHAR        DEFAULT NULL,
    desc_compl VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m410
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    nat_rec    VARCHAR        DEFAULT NULL,
    vl_rec     DECIMAL(21, 2) DEFAULT NULL,
    cod_cta    VARCHAR        DEFAULT NULL,
    desc_compl VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m500
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    cod_cred          VARCHAR        DEFAULT NULL,
    ind_cred_ori      TEXT           DEFAULT NULL,
    vl_bc_cred        DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins       DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins   DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant DECIMAL(23, 4) DEFAULT NULL,
    vl_cred           DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_acres     DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc     DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_dif       DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_disp      DECIMAL(21, 2) DEFAULT NULL,
    ind_desc_cred     TEXT           DEFAULT NULL,
    vl_cred_desc      DECIMAL(21, 2) DEFAULT NULL,
    sld_cred          DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m505
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    file_id             INTEGER,
    parent_id           INTEGER        DEFAULT NULL,
    reg                 TEXT           DEFAULT NULL,
    nat_bc_cred         VARCHAR        DEFAULT NULL,
    cst_cofins          VARCHAR        DEFAULT NULL,
    vl_bc_cofins_tot    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins_cum    DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins_nc     DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cofins        DECIMAL(21, 2) DEFAULT NULL,
    quant_bc_cofins_tot DECIMAL(22, 3) DEFAULT NULL,
    quant_bc_cofins     DECIMAL(22, 3) DEFAULT NULL,
    desc_cred           VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m510
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    ind_aj    TEXT           DEFAULT NULL,
    vl_aj     DECIMAL(21, 2) DEFAULT NULL,
    cod_aj    VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    descr_aj  VARCHAR        DEFAULT NULL,
    dt_ref    TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m600
(
    id                   INTEGER PRIMARY KEY NOT NULL,
    file_id              INTEGER,
    parent_id            INTEGER        DEFAULT NULL,
    reg                  TEXT           DEFAULT NULL,
    vl_tot_cont_nc_per   DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cred_desc     DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cred_desc_ant DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_nc_dev   DECIMAL(21, 2) DEFAULT NULL,
    vl_ret_nc            DECIMAL(21, 2) DEFAULT NULL,
    vl_out_ded_nc        DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_nc_rec       DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_cum_per  DECIMAL(21, 2) DEFAULT NULL,
    vl_ret_cum           DECIMAL(21, 2) DEFAULT NULL,
    vl_out_ded_cum       DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_cum_rec      DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_rec      DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m605
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    num_campo VARCHAR        DEFAULT NULL,
    cod_rec   VARCHAR        DEFAULT NULL,
    vl_debito DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m610
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    file_id                 INTEGER,
    parent_id               INTEGER        DEFAULT NULL,
    reg                     TEXT           DEFAULT NULL,
    cod_cont                VARCHAR        DEFAULT NULL,
    vl_rec_brt              DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont              DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_acres_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc_bc_cofins DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont_ajus         DECIMAL(21, 2) DEFAULT NULL,
    aliq_cofins             DECIMAL(12, 4) DEFAULT NULL,
    quant_bc_cofins         DECIMAL(22, 3) DEFAULT NULL,
    aliq_cofins_quant       DECIMAL(23, 4) DEFAULT NULL,
    vl_cont_apur            DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_acres           DECIMAL(21, 2) DEFAULT NULL,
    vl_ajus_reduc           DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer           DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer_ant       DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_per             DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m611
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    file_id                 INTEGER,
    parent_id               INTEGER        DEFAULT NULL,
    reg                     TEXT           DEFAULT NULL,
    ind_tip_coop            VARCHAR        DEFAULT NULL,
    vl_bc_cont_ant_exc_coop DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_coop_ger         DECIMAL(21, 2) DEFAULT NULL,
    vl_exc_esp_coop         DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont              DECIMAL(21, 2) DEFAULT NULL
);

CREATE TABLE efd_m620
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    ind_aj    TEXT           DEFAULT NULL,
    vl_aj     DECIMAL(21, 2) DEFAULT NULL,
    cod_aj    VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    descr_aj  VARCHAR        DEFAULT NULL,
    dt_ref    TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m630
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER,
    parent_id    INTEGER        DEFAULT NULL,
    reg          TEXT           DEFAULT NULL,
    cnpj         VARCHAR        DEFAULT NULL,
    vl_vend      DECIMAL(21, 2) DEFAULT NULL,
    vl_nao_receb DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_dif  DECIMAL(21, 2) DEFAULT NULL,
    vl_cred_dif  DECIMAL(21, 2) DEFAULT NULL,
    cod_cred     VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m700
(
    id                 INTEGER PRIMARY KEY NOT NULL,
    file_id            INTEGER,
    parent_id          INTEGER        DEFAULT NULL,
    reg                TEXT           DEFAULT NULL,
    cod_cont           VARCHAR        DEFAULT NULL,
    vl_cont_apur_difer DECIMAL(21, 2) DEFAULT NULL,
    nat_bc_cred_desc   VARCHAR        DEFAULT NULL,
    vl_cred_desc_difer DECIMAL(21, 2) DEFAULT NULL,
    vl_cont_difer_ant  DECIMAL(21, 2) DEFAULT NULL,
    per_apur           VARCHAR        DEFAULT NULL,
    dt_receb           TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_m800
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    cst_cofins VARCHAR        DEFAULT NULL,
    vl_tot_rec DECIMAL(21, 2) DEFAULT NULL,
    cod_cta    VARCHAR        DEFAULT NULL,
    desc_compl VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m810
(
    id         INTEGER PRIMARY KEY NOT NULL,
    file_id    INTEGER,
    parent_id  INTEGER        DEFAULT NULL,
    reg        TEXT           DEFAULT NULL,
    nat_rec    VARCHAR        DEFAULT NULL,
    vl_rec     DECIMAL(21, 2) DEFAULT NULL,
    cod_cta    VARCHAR        DEFAULT NULL,
    desc_compl VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_m990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_m VARCHAR DEFAULT NULL
);

CREATE TABLE efd_p001
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    ind_mov   TEXT    DEFAULT NULL
);

CREATE TABLE efd_p010
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    cnpj      VARCHAR DEFAULT NULL
);

CREATE TABLE efd_p100
(
    id                INTEGER PRIMARY KEY NOT NULL,
    file_id           INTEGER,
    parent_id         INTEGER        DEFAULT NULL,
    reg               TEXT           DEFAULT NULL,
    dt_ini            TEXT           DEFAULT NULL, /* Previously DATE */
    dt_fim            TEXT           DEFAULT NULL, /* Previously DATE */
    vl_rec_tot_est    DECIMAL(21, 2) DEFAULT NULL,
    cod_ativ_econ     VARCHAR        DEFAULT NULL,
    vl_rec_ativ_estab DECIMAL(21, 2) DEFAULT NULL,
    vl_exc            DECIMAL(21, 2) DEFAULT NULL,
    vl_bc_cont        DECIMAL(21, 2) DEFAULT NULL,
    aliq_cont         DECIMAL(12, 4) DEFAULT NULL,
    vl_cont_apu       DECIMAL(21, 2) DEFAULT NULL,
    cod_cta           VARCHAR        DEFAULT NULL,
    info_compl        VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_p110
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    num_campo VARCHAR        DEFAULT NULL,
    cod_det   VARCHAR        DEFAULT NULL,
    det_valor DECIMAL(21, 2) DEFAULT NULL,
    inf_compl VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_p199
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    num_proc  VARCHAR DEFAULT NULL,
    ind_proc  TEXT    DEFAULT NULL
);

CREATE TABLE efd_p200
(
    id              INTEGER PRIMARY KEY NOT NULL,
    file_id         INTEGER,
    parent_id       INTEGER        DEFAULT NULL,
    reg             TEXT           DEFAULT NULL,
    per_ref         VARCHAR        DEFAULT NULL,
    vl_tot_cont_apu DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_aj_reduc DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_aj_acres DECIMAL(21, 2) DEFAULT NULL,
    vl_tot_cont_dev DECIMAL(21, 2) DEFAULT NULL,
    cod_rec         VARCHAR        DEFAULT NULL
);

CREATE TABLE efd_p210
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER        DEFAULT NULL,
    reg       TEXT           DEFAULT NULL,
    ind_aj    TEXT           DEFAULT NULL,
    vl_aj     DECIMAL(21, 2) DEFAULT NULL,
    cod_aj    VARCHAR        DEFAULT NULL,
    num_doc   VARCHAR        DEFAULT NULL,
    descr_aj  VARCHAR        DEFAULT NULL,
    dt_ref    TEXT           DEFAULT NULL /* Previously DATE */
);

CREATE TABLE efd_p990
(
    id        INTEGER PRIMARY KEY NOT NULL,
    file_id   INTEGER,
    parent_id INTEGER DEFAULT NULL,
    reg       TEXT    DEFAULT NULL,
    qtd_lin_p VARCHAR DEFAULT NULL
);