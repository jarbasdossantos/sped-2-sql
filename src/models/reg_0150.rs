use std::future::Future;
use std::pin::Pin;
use sqlx::{FromRow, SqlitePool};

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0150 {
    pub parent_id: Option<i64>,
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

impl Reg0150 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>) -> Self {
        Reg0150 {
            parent_id,
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
}

impl Reg for Reg0150 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_part.as_deref(),
            self.nome.as_deref(),
            self.cod_pais.as_deref(),
            self.cnpj.as_deref(),
            self.cpf.as_deref(),
            self.ie.as_deref(),
            self.cod_mun.as_deref(),
            self.suframa.as_deref(),
            self.end.as_deref(),
            self.num.as_deref(),
            self.compl.as_deref(),
            self.bairro.as_deref(),
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
            sqlx::query("INSERT INTO reg_0150 (parent_id, reg, cod_part, nome, cod_pais, cnpj, cpf, ie, cod_mun, suframa, end, num, compl, bairro) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
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
                .execute(conn).await
        })
    }
}
