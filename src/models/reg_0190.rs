use super::traits::{Model, Reg};
use super::utils::get_field;
use crate::database::DB_POOL;
use crate::utils::database;
use indexmap::IndexMap;
use sqlx::{FromRow, Row};
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

static DB_FIELDS: &'static [&'static str] = &["ID", "FILE_ID", "PARENT_ID", "REG", "UNID", "DESCR"];
static TABLE: &str = "reg_0190";

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0190 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub unid: Option<String>,
    pub descr: Option<String>,
}

#[async_trait]
impl Model for Reg0190 {
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0190 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            unid: get_field(&fields, 2),
            descr: get_field(&fields, 3),
        }
    }

    async fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error> {
        let parent_id = parent_id.unwrap_or(0);

        {
            let data_vec = sqlx::query(
                format!(
                    "SELECT {} FROM {TABLE} WHERE FILE_ID = {file_id} AND PARENT_ID = {parent_id}",
                    DB_FIELDS.join(", ")
                )
                .as_str(),
            )
            .fetch_all(&*DB_POOL)
            .await?;

            let mut result = Vec::new();

            for data in data_vec {
                let parent_id: Option<i64> =
                    data.try_get::<i64, _>("PARENT_ID").unwrap_or(0).into();

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

                result.push(Self::new(
                    fields[2..].to_vec(),
                    fields.get(0).and_then(|v| v.parse().ok()),
                    parent_id,
                    file_id,
                ));
            }

            Ok(result)
        }
    }
}

impl Reg for Reg0190 {
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
            .bind(self.reg.as_deref())
            .bind(self.unid.as_deref())
            .bind(self.descr.as_deref())
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
            ("unid", self.unid.clone()),
            ("descr", self.descr.clone()),
        ])
    }
}
