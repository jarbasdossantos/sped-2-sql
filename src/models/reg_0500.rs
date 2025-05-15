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
    "DT_ALT",
    "COD_NAT_CC",
    "IND_CTA",
    "NIVEL",
    "COD_CTA",
    "NOME_CTA",
    "COD_CTA_REF",
    "CNPJ_EST",
];
static TABLE: &str = "reg_0500";

#[derive(Debug)]
pub struct Reg0500 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub dt_alt: Option<String>,
    pub cod_nat_cc: Option<String>,
    pub ind_cta: Option<String>,
    pub nivel: Option<String>,
    pub cod_cta: Option<String>,
    pub nome_cta: Option<String>,
    pub cod_cta_ref: Option<String>,
    pub cnpj_est: Option<String>,
}

#[async_trait]
impl Model for Reg0500 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0500 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            dt_alt: get_field(&fields, 2),
            cod_nat_cc: get_field(&fields, 3),
            ind_cta: get_field(&fields, 4),
            nivel: get_field(&fields, 5),
            cod_cta: get_field(&fields, 6),
            nome_cta: get_field(&fields, 7),
            cod_cta_ref: get_field(&fields, 8),
            cnpj_est: get_field(&fields, 9),
        }
    }
}

impl Reg for Reg0500 {
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
            .bind(&self.dt_alt)
            .bind(&self.cod_nat_cc)
            .bind(&self.ind_cta)
            .bind(&self.nivel)
            .bind(&self.cod_cta)
            .bind(&self.nome_cta)
            .bind(&self.cod_cta_ref)
            .bind(&self.cnpj_est)
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
            ("dt_alt", self.dt_alt.clone()),
            ("cod_nat_cc", self.cod_nat_cc.clone()),
            ("ind_cta", self.ind_cta.clone()),
            ("nivel", self.nivel.clone()),
            ("cod_cta", self.cod_cta.clone()),
            ("nome_cta", self.nome_cta.clone()),
            ("cod_cta_ref", self.cod_cta_ref.clone()),
            ("cnpj_est", self.cnpj_est.clone()),
        ])
    }
}
