use std::future::Future;
use std::pin::Pin;
use sqlx::{FromRow, SqlitePool};

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0035 {
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_scp: Option<String>,
    pub nome_scp: Option<String>,
    pub inf_comp: Option<String>,
}

impl Reg0035 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>) -> Self {
        Reg0035 {
            parent_id,
            reg: get_field(&fields, 1),
            cod_scp: get_field(&fields, 2),
            nome_scp: get_field(&fields, 3),
            inf_comp: get_field(&fields, 4),
        }
    }
}

impl Reg for Reg0035 {
    fn to_line(&self) -> String {
        let fields = [self.reg.as_deref(), self.cod_scp.as_deref(), self.nome_scp.as_deref(), self.inf_comp.as_deref()];

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
            sqlx::query("INSERT INTO reg_0035 (PARENT_ID, REG, COD_SCP, NOME_SCP, INF_COMP) VALUES (?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.reg)
                .bind(&self.cod_scp)
                .bind(&self.nome_scp)
                .bind(&self.inf_comp)
                .execute(conn).await
        })
    }
}
