use crate::database::DB_POOL;
use crate::models::traits::{Model, Reg};
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
    "PER_REF",
    "VL_TOT_CONT_APU",
    "VL_TOT_AJ_REDUC",
    "VL_TOT_AJ_ACRES",
    "VL_TOT_CONT_DEV",
    "COD_REC",
];
static TABLE: &str = "reg_p200";

#[derive(Debug)]
pub struct RegP200 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub per_ref: Option<String>,
    pub vl_tot_cont_apu: Option<String>,
    pub vl_tot_aj_reduc: Option<String>,
    pub vl_tot_aj_acres: Option<String>,
    pub vl_tot_cont_dev: Option<String>,
    pub cod_rec: Option<String>,
}

#[async_trait]
impl Model for RegP200 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegP200 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            per_ref: get_field(&fields, 2),
            vl_tot_cont_apu: get_field(&fields, 3),
            vl_tot_aj_reduc: get_field(&fields, 4),
            vl_tot_aj_acres: get_field(&fields, 5),
            vl_tot_cont_dev: get_field(&fields, 6),
            cod_rec: get_field(&fields, 7),
        }
    }
}

impl Reg for RegP200 {
    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        let id: Option<String> = self.id.clone().map(|id| id.to_string());
        let parent_id: Option<String> = self.parent_id.map(|id| id.to_string());

        IndexMap::from([
            ("id", id),
            ("file_id", Some(self.file_id.to_string())),
            ("parent_id", parent_id),
            ("reg", self.reg.clone()),
            ("per_ref", self.per_ref.clone()),
            ("vl_tot_cont_apu", self.vl_tot_cont_apu.clone()),
            ("vl_tot_aj_reduc", self.vl_tot_aj_reduc.clone()),
            ("vl_tot_aj_acres", self.vl_tot_aj_acres.clone()),
            ("vl_tot_cont_dev", self.vl_tot_cont_dev.clone()),
            ("cod_rec", self.cod_rec.clone()),
        ])
    }

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
            .bind(self.file_id)
            .bind(self.parent_id)
            .bind(&self.reg)
            .bind(&self.per_ref)
            .bind(&self.vl_tot_cont_apu)
            .bind(&self.vl_tot_aj_reduc)
            .bind(&self.vl_tot_aj_acres)
            .bind(&self.vl_tot_cont_dev)
            .bind(&self.cod_rec)
            .execute(&*DB_POOL)
            .await
        })
    }
}
