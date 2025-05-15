use super::traits::{Model, Reg};
use super::utils::get_field;
use crate::database::DB_POOL;
use crate::utils::database;
use async_trait::async_trait;
use indexmap::IndexMap;
use sqlx::FromRow;
use std::future::Future;
use std::pin::Pin;

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
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

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
