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
    "COD_EST",
    "NOME",
    "CNPJ",
    "UF",
    "IE",
    "COD_MUN",
    "IM",
    "SUFRAMA",
];

static TABLE: &str = "reg_0140";

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0140 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_est: Option<String>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub uf: Option<String>,
    pub ie: Option<String>,
    pub cod_mun: Option<String>,
    pub im: Option<String>,
    pub suframa: Option<String>,
}

#[async_trait]
impl Model for Reg0140 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0140 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            cod_est: get_field(&fields, 2),
            nome: get_field(&fields, 3),
            cnpj: get_field(&fields, 4),
            uf: get_field(&fields, 5),
            ie: get_field(&fields, 6),
            cod_mun: get_field(&fields, 7),
            im: get_field(&fields, 8),
            suframa: get_field(&fields, 9),
        }
    }
}

impl Reg for Reg0140 {
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
            .bind(&self.cod_est)
            .bind(&self.nome)
            .bind(&self.cnpj)
            .bind(&self.uf)
            .bind(&self.ie)
            .bind(&self.cod_mun)
            .bind(&self.im)
            .bind(&self.suframa)
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
            ("cod_est", self.cod_est.clone()),
            ("nome", self.nome.clone()),
            ("cnpj", self.cnpj.clone()),
            ("uf", self.uf.clone()),
            ("ie", self.ie.clone()),
            ("cod_mun", self.cod_mun.clone()),
            ("im", self.im.clone()),
            ("suframa", self.suframa.clone()),
        ])
    }
}
