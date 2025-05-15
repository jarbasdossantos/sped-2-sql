use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use async_trait::async_trait;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "CST_PIS",
    "CFOP",
    "VL_ITEM",
    "VL_DESC",
    "VL_BC_PIS",
    "ALIQ_PIS",
    "QUANT_BC_PIS",
    "ALIQ_PIS_QUANT",
    "VL_PIS",
    "COD_CTA",
];
static TABLE: &str = "reg_C181";

#[derive(Debug)]
pub struct RegC181 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cst_pis: Option<String>,
    pub cfop: Option<String>,
    pub vl_item: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for RegC181 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegC181 {
            id,
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            cst_pis: get_field(&fields, 2),
            cfop: get_field(&fields, 3),
            vl_item: get_field(&fields, 4),
            vl_desc: get_field(&fields, 5),
            vl_bc_pis: get_field(&fields, 6),
            aliq_pis: get_field(&fields, 7),
            quant_bc_pis: get_field(&fields, 8),
            aliq_pis_quant: get_field(&fields, 9),
            vl_pis: get_field(&fields, 10),
            cod_cta: get_field(&fields, 11),
        }
    }
}

impl Reg for RegC181 {
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
            .bind(&self.cst_pis)
            .bind(&self.cfop)
            .bind(&self.vl_item)
            .bind(&self.vl_desc)
            .bind(&self.vl_bc_pis)
            .bind(&self.aliq_pis)
            .bind(&self.quant_bc_pis)
            .bind(&self.aliq_pis_quant)
            .bind(&self.vl_pis)
            .bind(&self.cod_cta)
            .execute(&*DB_POOL)
            .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        let id: Option<String> = self.id.clone().map(|id| id.to_string());
        let parent_id: Option<String> = self.parent_id.map(|id| id.to_string());

        IndexMap::from([
            ("id", id),
            ("file_id", Some(self.file_id.to_string())),
            ("parent_id", parent_id),
            ("reg", self.reg.clone()),
            ("cst_pis", self.cst_pis.clone()),
            ("cfop", self.cfop.clone()),
            ("vl_item", self.vl_item.clone()),
            ("vl_desc", self.vl_desc.clone()),
            ("vl_bc_pis", self.vl_bc_pis.clone()),
            ("aliq_pis", self.aliq_pis.clone()),
            ("quant_bc_pis", self.quant_bc_pis.clone()),
            ("aliq_pis_quant", self.aliq_pis_quant.clone()),
            ("vl_pis", self.vl_pis.clone()),
            ("cod_cta", self.cod_cta.clone()),
        ])
    }
}
