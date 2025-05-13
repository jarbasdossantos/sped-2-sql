use super::traits::{Model, Reg};
use super::utils::get_field;
use crate::database::DB_POOL;
use crate::utils::database;
use async_trait::async_trait;
use indexmap::IndexMap;
use sqlx::FromRow;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "IND_OPER",
    "IND_EMIT",
    "COD_PART",
    "COD_MOD",
    "COD_SIT",
    "SER",
    "NUM_DOC",
    "CHV_NFE",
    "DT_DOC",
    "DT_E_S",
    "VL_DOC",
    "IND_PGTO",
    "VL_DESC",
    "VL_ABAT_NT",
    "VL_MERC",
    "IND_FRT",
    "VL_FRT",
    "VL_SEG",
    "VL_OUT_DA",
    "VL_BC_ICMS",
    "VL_ICMS",
    "VL_BC_ICMS_ST",
    "VL_ICMS_ST",
    "VL_IPI",
    "VL_PIS",
    "VL_COFINS",
    "VL_PIS_ST",
    "VL_COFINS_ST",
];
static TABLE: &str = "reg_c100";

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct RegC100 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub ind_oper: Option<String>,
    pub ind_emit: Option<String>,
    pub cod_part: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_sit: Option<String>,
    pub ser: Option<String>,
    pub num_doc: Option<String>,
    pub chv_nfe: Option<String>,
    pub dt_doc: Option<String>,
    pub dt_e_s: Option<String>,
    pub vl_doc: Option<String>,
    pub ind_pgto: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_abat_nt: Option<String>,
    pub vl_merc: Option<String>,
    pub ind_frt: Option<String>,
    pub vl_frt: Option<String>,
    pub vl_seg: Option<String>,
    pub vl_out_da: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub vl_ipi: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
    pub vl_pis_st: Option<String>,
    pub vl_cofins_st: Option<String>,
    pub vl_desc_compl: Option<f64>,
}

#[async_trait]
impl Model for RegC100 {
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        let vl_desc_compl = get_field(&fields, 30).and_then(|s| s.parse::<f64>().ok());

        RegC100 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            ind_oper: get_field(&fields, 2),
            ind_emit: get_field(&fields, 3),
            cod_part: get_field(&fields, 4),
            cod_mod: get_field(&fields, 5),
            cod_sit: get_field(&fields, 6),
            ser: get_field(&fields, 7),
            num_doc: get_field(&fields, 8),
            chv_nfe: get_field(&fields, 9),
            dt_doc: get_field(&fields, 10),
            dt_e_s: get_field(&fields, 11),
            vl_doc: get_field(&fields, 12),
            ind_pgto: get_field(&fields, 13),
            vl_desc: get_field(&fields, 14),
            vl_abat_nt: get_field(&fields, 15),
            vl_merc: get_field(&fields, 16),
            ind_frt: get_field(&fields, 17),
            vl_frt: get_field(&fields, 18),
            vl_seg: get_field(&fields, 19),
            vl_out_da: get_field(&fields, 20),
            vl_bc_icms: get_field(&fields, 21),
            vl_icms: get_field(&fields, 22),
            vl_bc_icms_st: get_field(&fields, 23),
            vl_icms_st: get_field(&fields, 24),
            vl_ipi: get_field(&fields, 25),
            vl_pis: get_field(&fields, 26),
            vl_cofins: get_field(&fields, 27),
            vl_pis_st: get_field(&fields, 28),
            vl_cofins_st: get_field(&fields, 29),
            vl_desc_compl,
        }
    }

    fn load<'a>(file_id: i64, parent_id: Option<i64>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + 'a>> {
        todo!()
    }
}

impl Reg for RegC100 {
    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query(
                format!(
                    "INSERT INTO {TABLE} ({}) VALUES ({})",
                    DB_FIELDS[1..].join(", "),
                    database::binds(DB_FIELDS.len() - 1)
                )
                .as_str(),
            )
            .bind(&self.file_id)
            .bind(&self.parent_id)
            .bind(&self.reg)
            .bind(&self.ind_oper)
            .bind(&self.ind_emit)
            .bind(&self.cod_part)
            .bind(&self.cod_mod)
            .bind(&self.cod_sit)
            .bind(&self.ser)
            .bind(&self.num_doc)
            .bind(&self.chv_nfe)
            .bind(&self.dt_doc)
            .bind(&self.dt_e_s)
            .bind(&self.vl_doc)
            .bind(&self.ind_pgto)
            .bind(&self.vl_desc)
            .bind(&self.vl_abat_nt)
            .bind(&self.vl_merc)
            .bind(&self.ind_frt)
            .bind(&self.vl_frt)
            .bind(&self.vl_seg)
            .bind(&self.vl_out_da)
            .bind(&self.vl_bc_icms)
            .bind(&self.vl_icms)
            .bind(&self.vl_bc_icms_st)
            .bind(&self.vl_icms_st)
            .bind(&self.vl_ipi)
            .bind(&self.vl_pis)
            .bind(&self.vl_cofins)
            .bind(&self.vl_pis_st)
            .bind(&self.vl_cofins_st)
            .execute(&*DB_POOL)
            .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        IndexMap::from([
            ("id", self.id.map(|id| id.to_string())),
            ("file_id", Some(self.file_id.to_string())),
            ("parent_id", self.parent_id.map(|id| id.to_string())),
            ("reg", Some("0140".to_string())),
            ("ind_oper", self.ind_oper.clone()),
            ("ind_emit", self.ind_emit.clone()),
            ("cod_part", self.cod_part.clone()),
            ("cod_mod", self.cod_mod.clone()),
            ("cod_sit", self.cod_sit.clone()),
            ("ser", self.ser.clone()),
            ("num_doc", self.num_doc.clone()),
            ("chv_nfe", self.chv_nfe.clone()),
            ("dt_doc", self.dt_doc.clone()),
            ("dt_e_s", self.dt_e_s.clone()),
            ("vl_doc", self.vl_doc.clone()),
            ("ind_pgto", self.ind_pgto.clone()),
            ("vl_desc", self.vl_desc.clone()),
            ("vl_abat_nt", self.vl_abat_nt.clone()),
            ("vl_merc", self.vl_merc.clone()),
            ("ind_frt", self.ind_frt.clone()),
            ("vl_frt", self.vl_frt.clone()),
            ("vl_seg", self.vl_seg.clone()),
            ("vl_out_da", self.vl_out_da.clone()),
            ("vl_bc_icms", self.vl_bc_icms.clone()),
            ("vl_icms", self.vl_icms.clone()),
            ("vl_bc_icms_st", self.vl_bc_icms_st.clone()),
            ("vl_icms_st", self.vl_icms_st.clone()),
            ("vl_ipi", self.vl_ipi.clone()),
            ("vl_pis", self.vl_pis.clone()),
            ("vl_cofins", self.vl_cofins.clone()),
            ("vl_pis_st", self.vl_pis_st.clone()),
            ("vl_cofins_st", self.vl_cofins_st.clone()),
        ])
    }
}
