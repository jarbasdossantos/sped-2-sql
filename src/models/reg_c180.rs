use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "COD_MOD",
    "DT_DOC_INI",
    "DT_DOC_FIN",
    "COD_ITEM",
    "COD_NCM",
    "EX_IPI",
    "VL_TOT_ITEM",
];
static TABLE: &str = "reg_C180";

#[derive(Debug)]
pub struct RegC180 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub dt_doc_ini: Option<String>,
    pub dt_doc_fin: Option<String>,
    pub cod_item: Option<String>,
    pub cod_ncm: Option<String>,
    pub ex_ipi: Option<String>,
    pub vl_tot_item: Option<String>,
}

#[async_trait]
impl Model for RegC180 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegC180 {
            id,
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            cod_mod: get_field(&fields, 2),
            dt_doc_ini: get_field(&fields, 3),
            dt_doc_fin: get_field(&fields, 4),
            cod_item: get_field(&fields, 5),
            cod_ncm: get_field(&fields, 6),
            ex_ipi: get_field(&fields, 7),
            vl_tot_item: get_field(&fields, 8),
        }
    }
}

impl Reg for RegC180 {
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
            .bind(&self.cod_mod)
            .bind(&self.dt_doc_ini)
            .bind(&self.dt_doc_fin)
            .bind(&self.cod_item)
            .bind(&self.cod_ncm)
            .bind(&self.ex_ipi)
            .bind(&self.vl_tot_item)
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
            ("cod_mod", self.cod_mod.clone()),
            ("dt_doc_ini", self.dt_doc_ini.clone()),
            ("dt_doc_fin", self.dt_doc_fin.clone()),
            ("cod_item", self.cod_item.clone()),
            ("cod_ncm", self.cod_ncm.clone()),
            ("ex_ipi", self.ex_ipi.clone()),
            ("vl_tot_item", self.vl_tot_item.clone()),
        ])
    }
}
