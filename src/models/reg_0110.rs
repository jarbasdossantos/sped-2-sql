use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "COD_INC_TRIB",
    "IND_APRO_CRED",
    "COD_TIPO_CONT",
    "IND_REG_CUM",
];
static TABLE: &str = "reg_0110";

#[derive(Debug)]
pub struct Reg0110 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_inc_trib: Option<String>,
    pub ind_apro_cred: Option<String>,
    pub cod_tipo_cont: Option<String>,
    pub ind_reg_cum: Option<String>,
}

impl Reg0110 {
    pub fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0110 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            cod_inc_trib: get_field(&fields, 2),
            ind_apro_cred: get_field(&fields, 3),
            cod_tipo_cont: get_field(&fields, 4),
            ind_reg_cum: get_field(&fields, 5),
        }
    }
}

impl Reg for Reg0110 {
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
            .bind(&self.cod_inc_trib)
            .bind(&self.ind_apro_cred)
            .bind(&self.cod_tipo_cont)
            .bind(&self.ind_reg_cum)
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
            ("cod_inc_trib", self.cod_inc_trib.clone()),
            ("ind_apro_cred", self.ind_apro_cred.clone()),
            ("cod_tipo_cont", self.cod_tipo_cont.clone()),
            ("ind_reg_cum", self.ind_reg_cum.clone()),
        ])
    }
}
