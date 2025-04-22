use sqlx::{FromRow, SqlitePool};
use std::future::Future;
use std::pin::Pin;

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0190 {
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub unid: Option<String>,
    pub descr: Option<String>,
}

impl Reg0190 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0190 {
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            unid: get_field(&fields, 2),
            descr: get_field(&fields, 3),
        }
    }
}

impl Reg for Reg0190 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.unid.as_deref(),
            self.descr.as_deref(),
        ];

        format!(
            "|{}|",
            fields
                .iter()
                .map(|f| f.unwrap_or(""))
                .collect::<Vec<&str>>()
                .join("|")
        )
    }

    fn to_db<'a>(
        &'a self,
        conn: &'a SqlitePool,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query("INSERT INTO reg_0190 (PARENT_ID, FILE_ID, REG, UNID, DESCR) VALUES (?, ?, ?, ?, ?)")
                .bind(self.parent_id)
                .bind(self.file_id)
                .bind(self.reg.as_deref())
                .bind(self.unid.as_deref())
                .bind(self.descr.as_deref())
                .execute(conn).await
        })
    }
}
