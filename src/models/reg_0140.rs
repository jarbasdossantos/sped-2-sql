use super::traits::{Model, Reg};
use super::utils::get_field;
use crate::database::DB_POOL;
use futures::executor::block_on;
use indexmap::IndexMap;
use sqlx::{FromRow, Row};
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "PARENT_ID",
    "FILE_ID",
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
    pub parent_id: Option<i64>,
    pub file_id: i64,
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

impl Model for Reg0140 {
    fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0140 {
            id: fields.get(0).and_then(|v| v.parse().ok()),
            parent_id,
            file_id,
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

    fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error> {
        block_on(async move {
            let data_vec = sqlx::query(
                format!(
                    "SELECT {} FROM {TABLE} WHERE FILE_ID = ? AND PARENT_ID = ?",
                    DB_FIELDS.join(", ")
                )
                .as_str(),
            )
            .bind(file_id)
            .bind(parent_id.unwrap_or(0))
            .fetch_all(&*DB_POOL)
            .await?;

            let mut result = Vec::new();

            for data in data_vec {
                let parent_id = data.try_get::<i64, _>("PARENT_ID").unwrap_or(0).into();

                let _fields: Vec<String> = DB_FIELDS
                    .iter()
                    .map(|field| {
                        if vec!["ID", "FILE_ID"].contains(field) {
                            data.try_get::<i64, _>(*field).unwrap_or(0).to_string()
                        } else {
                            data.try_get::<String, _>(*field).unwrap_or("".to_string())
                        }
                    })
                    .collect();

                let fields: Vec<&str> = _fields.iter().map(|field| field.as_str()).collect();

                result.push(Self::new(fields, parent_id, file_id));
            }

            Ok(result)
        })
    }
}

impl Reg for Reg0140 {
    fn to_line(&self) -> String {
        format!(
            "|{}|",
            self.values()
                .iter()
                .skip(2)
                .map(|(_, v)| v.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query("INSERT INTO {TABLE} ({}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.file_id)
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
        let mut values = IndexMap::new();

        values.insert("id", self.id.map(|id| id.to_string()));
        values.insert("parent_id", self.parent_id.map(|id| id.to_string()));
        values.insert("reg", Some("0140".to_string()));
        values.insert("cod_est", self.cod_est.clone());
        values.insert("nome", self.nome.clone());
        values.insert("cnpj", self.cnpj.clone());
        values.insert("uf", self.uf.clone());
        values.insert("ie", self.ie.clone());
        values.insert("cod_mun", self.cod_mun.clone());
        values.insert("im", self.im.clone());
        values.insert("suframa", self.suframa.clone());

        values
    }
}
