use std::future::Future;
use std::pin::Pin;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, SqlitePool};
use crate::models::reg_trait::Reg;
use crate::models::utils::get_field;

#[derive(Debug)]
pub struct Reg0100 {
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub nome: Option<String>,
    pub cpf: Option<String>,
    pub crc: Option<String>,
    pub cnpj: Option<String>,
    pub cep: Option<String>,
    pub end: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
    pub fone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub cod_mun: Option<String>,
}

impl Reg0100 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>) -> Self {
        Reg0100 {
            parent_id,
            reg: get_field(&fields, 1),
            nome: get_field(&fields, 2),
            cpf: get_field(&fields, 3),
            crc: get_field(&fields, 4),
            cnpj: get_field(&fields, 5),
            cep: get_field(&fields, 6),
            end: get_field(&fields, 7),
            num: get_field(&fields, 8),
            compl: get_field(&fields, 9),
            bairro: get_field(&fields, 10),
            fone: get_field(&fields, 11),
            fax: get_field(&fields, 12),
            email: get_field(&fields, 13),
            cod_mun: get_field(&fields, 14),
        }
    }
}

impl Reg for Reg0100 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.nome.as_deref(),
            self.cpf.as_deref(),
            self.crc.as_deref(),
            self.cnpj.as_deref(),
            self.cep.as_deref(),
            self.end.as_deref(),
            self.num.as_deref(),
            self.compl.as_deref(),
            self.bairro.as_deref(),
            self.fone.as_deref(),
            self.fax.as_deref(),
            self.email.as_deref(),
            self.cod_mun.as_deref(),
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

    fn to_db<'a>(&'a self, conn: &'a SqlitePool) -> Pin<Box<dyn Future<Output=Result<SqliteQueryResult, Error>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query("INSERT INTO reg_0100 (PARENT_ID, REG, NOME, CPF, CRC, CNPJ, CEP, END, NUM, COMPL, BAIRRO, FONE, FAX, EMAIL, COD_MUN) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.reg)
                .bind(&self.nome)
                .bind(&self.cpf)
                .bind(&self.crc)
                .bind(&self.cnpj)
                .bind(&self.cep)
                .bind(&self.end)
                .bind(&self.num)
                .bind(&self.compl)
                .bind(&self.bairro)
                .bind(&self.fone)
                .bind(&self.fax)
                .bind(&self.email)
                .bind(&self.cod_mun)
                .execute(conn).await
        })
    }
}
