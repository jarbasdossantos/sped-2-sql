use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "NOME",
    "CPF",
    "CRC",
    "CNPJ",
    "CEP",
    "END",
    "NUM",
    "COMPL",
    "BAIRRO",
    "FONE",
    "FAX",
    "EMAIL",
    "COD_MUN",
];
static TABLE: &str = "reg_0100";

#[derive(Debug)]
pub struct Reg0100 {
    pub id: Option<i64>,
    pub file_id: i64,
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
    pub fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0100 {
            id,
            file_id,
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
            .execute(&*DB_POOL)
            .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        todo!()
    }
}
