use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use futures::executor::block_on;
use indexmap::IndexMap;
use sqlx::Row;
use std::future::Future;
use std::pin::Pin;

use super::traits::Model;

static DB_FIELDS: &'static [&'static str] = &["ID", "FILE_ID", "PARENT_ID", "REG", "IND_MOV"];
static TABLE: &str = "reg_0001";

#[derive(Debug)]
pub struct Reg0001 {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub ind_mov: Option<String>,
}

impl Model for Reg0001 {
    fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0001 {
            id: fields.get(0).and_then(|v| v.parse().ok()),
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            ind_mov: get_field(&fields, 2),
        }
    }

    fn load(file_id: i64, parent_id: Option<i64>) -> anyhow::Result<Vec<Self>, anyhow::Error> {
        block_on(async move {
            let rows = sqlx::query(
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

            let mut data = Vec::new();

            for row in rows {
                let _fields: Vec<String> = DB_FIELDS
                    .iter()
                    .map(|field| {
                        if vec!["ID", "FILE_ID"].contains(field) {
                            row.try_get::<i64, _>(*field).unwrap_or(0).to_string()
                        } else {
                            row.try_get::<String, _>(*field).unwrap_or("".to_string())
                        }
                    })
                    .collect();

                let fields: Vec<&str> = _fields.iter().map(|field| field.as_str()).collect();

                data.push(Self::new(fields, Some(0i64), file_id));
            }

            Ok(data)
        })
    }
}

impl Reg for Reg0001 {
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
            sqlx::query(
                format!(
                    "INSERT INTO {TABLE} ({}) VALUES (?, ?, ?, ?)",
                    DB_FIELDS.join(", ")
                )
                .as_str(),
            )
            .bind(&self.parent_id)
            .bind(&self.file_id)
            .bind(&self.reg)
            .bind(&self.ind_mov)
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
            ("reg", self.reg.clone()),
            ("ind_mov", self.ind_mov.clone()),
        ])
    }
}
