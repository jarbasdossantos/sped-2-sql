use super::traits::{Model, Reg};
use super::utils::get_field;
use crate::database::DB_POOL;
use futures::executor::block_on;
use indexmap::IndexMap;
use sqlx::{FromRow, Row};
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &[&str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "COD_PART",
    "NOME",
    "COD_PAIS",
    "CNPJ",
    "CPF",
    "IE",
    "COD_MUN",
    "SUFRAMA",
    "END",
    "NUM",
    "COMPL",
    "BAIRRO",
];
static TABLE: &str = "reg_0150";

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0150 {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub cod_part: Option<String>,
    pub nome: Option<String>,
    pub cod_pais: Option<String>,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub ie: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub end: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
}

impl Model for Reg0150 {
    fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0150 {
            id: fields.get(0).and_then(|v| v.parse().ok()),
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            cod_part: get_field(&fields, 2),
            nome: get_field(&fields, 3),
            cod_pais: get_field(&fields, 4),
            cnpj: get_field(&fields, 5),
            cpf: get_field(&fields, 6),
            ie: get_field(&fields, 7),
            cod_mun: get_field(&fields, 8),
            suframa: get_field(&fields, 9),
            end: get_field(&fields, 10),
            num: get_field(&fields, 11),
            compl: get_field(&fields, 12),
            bairro: get_field(&fields, 13),
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
                        if vec!["ID", "PARENT_ID", "FILE_ID"].contains(field) {
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

impl Reg for Reg0150 {
    fn to_line(&self) -> String {
        format!(
            "|{}|",
            self.values()
                .iter()
                .skip(3)
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
            sqlx::query(
                "INSERT INTO {TABLE} ({}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&self.parent_id)
            .bind(&self.file_id)
            .bind(&self.reg)
            .bind(&self.cod_part)
            .bind(&self.nome)
            .bind(&self.cod_pais)
            .bind(&self.cnpj)
            .bind(&self.cpf)
            .bind(&self.ie)
            .bind(&self.cod_mun)
            .bind(&self.suframa)
            .bind(&self.end)
            .bind(&self.num)
            .bind(&self.compl)
            .bind(&self.bairro)
            .execute(&*DB_POOL)
            .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        let id: Option<String> = self.id.clone().map(|id| id.to_string());
        let parent_id: Option<String> = self.parent_id.map(|id| id.to_string());

        IndexMap::from([
            ("id", id),
            ("parent_id", parent_id),
            ("file_id", Some(self.file_id.to_string())),
            ("reg", self.reg.clone()),
            ("cod_part", self.cod_part.clone()),
            ("nome", self.nome.clone()),
            ("cod_pais", self.cod_pais.clone()),
            ("cnpj", self.cnpj.clone()),
            ("cpf", self.cpf.clone()),
            ("ie", self.ie.clone()),
            ("cod_mun", self.cod_mun.clone()),
            ("suframa", self.suframa.clone()),
            ("end", self.end.clone()),
            ("num", self.num.clone()),
            ("compl", self.compl.clone()),
            ("bairro", self.bairro.clone()),
        ])
    }
}
