use crate::models::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use async_trait::async_trait;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Reg0001 {
    pub id: Option<i32>,
    pub file_id: i32,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_mov: Option<String>,
}

#[async_trait]
impl Model for Reg0001 {
    fn new(fields: Vec<&str>, id: Option<i32>, parent_id: Option<i32>, file_id: i32) -> Self {
        Reg0001 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            ind_mov: get_field(&fields, 2),
        }
    }
}

impl Reg for Reg0001 {
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
            ("file_id", Some(self.file_id.to_string())),
            ("parent_id", parent_id),
            ("reg", self.reg.clone()),
            ("ind_mov", self.ind_mov.clone()),
        ])
    }
}
