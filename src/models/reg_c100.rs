use super::traits::Reg;
use super::utils::get_field;
use crate::database::DB_POOL;
use indexmap::IndexMap;
use sqlx::FromRow;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct RegC100 {
    pub parent_id: Option<i64>,
    pub file_id: i64,
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
}

impl RegC100 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegC100 {
            parent_id,
            file_id,
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
        }
    }
}

impl Reg for RegC100 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.ind_oper.as_deref(),
            self.ind_emit.as_deref(),
            self.cod_part.as_deref(),
            self.cod_mod.as_deref(),
            self.cod_sit.as_deref(),
            self.ser.as_deref(),
            self.num_doc.as_deref(),
            self.chv_nfe.as_deref(),
            self.dt_doc.as_deref(),
            self.dt_e_s.as_deref(),
            self.vl_doc.as_deref(),
            self.ind_pgto.as_deref(),
            self.vl_desc.as_deref(),
            self.vl_abat_nt.as_deref(),
            self.vl_merc.as_deref(),
            self.ind_frt.as_deref(),
            self.vl_frt.as_deref(),
            self.vl_seg.as_deref(),
            self.vl_out_da.as_deref(),
            self.vl_bc_icms.as_deref(),
            self.vl_icms.as_deref(),
            self.vl_bc_icms_st.as_deref(),
            self.vl_icms_st.as_deref(),
            self.vl_ipi.as_deref(),
            self.vl_pis.as_deref(),
            self.vl_cofins.as_deref(),
            self.vl_pis_st.as_deref(),
            self.vl_cofins_st.as_deref(),
        ];

        format!(
            "|{}|",
            fields
                .iter()
                .map(|f| f.unwrap_or(""))
                .collect::<Vec<&str>>()
                .join("|")
        )
    }

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query("INSERT INTO reg_c100 (PARENT_ID, FILE_ID, REG, IND_OPER, IND_EMIT, COD_PART, COD_MOD, COD_SIT, SER, NUM_DOC, CHV_NFE, DT_DOC, DT_E_S, VL_DOC, IND_PGTO, VL_DESC, VL_ABAT_NT, VL_MERC, IND_FRT, VL_FRT, VL_SEG, VL_OUT_DA, VL_BC_ICMS, VL_ICMS, VL_BC_ICMS_ST, VL_ICMS_ST, VL_IPI, VL_PIS, VL_COFINS, VL_PIS_ST, VL_COFINS_ST) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.file_id)
                .bind(&self.file_id)
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
                .execute(&*DB_POOL).await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        todo!()
    }
}
