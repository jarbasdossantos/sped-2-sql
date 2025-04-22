use std::future::Future;
use std::pin::Pin;
use sqlx::{SqlitePool};
use crate::models::reg_trait::Reg;
use crate::models::utils::{get_field};

#[derive(Debug)]
pub struct Reg0001 {
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub ind_mov: Option<String>,
}

impl Reg0001 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0001 {
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            ind_mov: get_field(&fields, 2),
        }
    }
}

impl Reg for Reg0001 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.ind_mov.as_deref(),
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

    fn to_db<'a>(&'a self, conn: &'a SqlitePool) -> Pin<Box<dyn Future<Output=Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query("INSERT INTO reg_0001 (PARENT_ID, FILE_ID, REG, IND_MOV) VALUES (?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.file_id)
                .bind(&self.reg)
                .bind(&self.ind_mov)
                .execute(conn).await
        })
    }
}
