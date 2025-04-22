use sqlx::{FromRow, SqlitePool};
use std::future::Future;
use std::pin::Pin;

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0140 {
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

impl Reg0140 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0140 {
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
}

impl Reg for Reg0140 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_est.as_deref(),
            self.nome.as_deref(),
            self.cnpj.as_deref(),
            self.uf.as_deref(),
            self.ie.as_deref(),
            self.cod_mun.as_deref(),
            self.im.as_deref(),
            self.suframa.as_deref(),
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
            sqlx::query("INSERT INTO reg_0140 (PARENT_ID, FILE_ID, REG, COD_EST, NOME, CNPJ, UF, IE, COD_MUN, IM, SUFRAMA) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
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
                .execute(conn).await
        })
    }
}
