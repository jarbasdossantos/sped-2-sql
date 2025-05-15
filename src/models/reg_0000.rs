use crate::database::DB_POOL;
use crate::models::traits::{Model, Reg};
use crate::models::utils::get_field;
use crate::utils::database;
use async_trait::async_trait;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "COD_VER",
    "TIPO_ESCRIT",
    "IND_SIT_ESP",
    "NUM_REC_ANTERIOR",
    "DT_INI",
    "DT_FIN",
    "NOME",
    "CNPJ",
    "UF",
    "COD_MUN",
    "SUFRAMA",
    "IND_NAT_PJ",
    "IND_ATIV",
];
static TABLE: &str = "reg_0000";

#[derive(Debug)]
pub struct Reg0000 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_ver: Option<String>,
    pub tipo_escrit: Option<String>,
    pub ind_sit_esp: Option<String>,
    pub num_rec_anterior: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fin: Option<String>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub uf: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub ind_nat_pj: Option<String>,
    pub ind_ativ: Option<String>,
}

#[async_trait]
impl Model for Reg0000 {
    fn table() -> &'static str {
        TABLE
    }

    fn fields() -> &'static [&'static str] {
        DB_FIELDS
    }

    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0000 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            cod_ver: get_field(&fields, 2),
            tipo_escrit: get_field(&fields, 3),
            ind_sit_esp: get_field(&fields, 4),
            num_rec_anterior: get_field(&fields, 5),
            dt_ini: get_field(&fields, 6),
            dt_fin: get_field(&fields, 7),
            nome: get_field(&fields, 8),
            cnpj: get_field(&fields, 9),
            uf: get_field(&fields, 10),
            cod_mun: get_field(&fields, 11),
            suframa: get_field(&fields, 12),
            ind_nat_pj: get_field(&fields, 13),
            ind_ativ: get_field(&fields, 14),
        }
    }
}

impl Reg for Reg0000 {
    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        let id: Option<String> = self.id.clone().map(|id| id.to_string());
        let parent_id: Option<String> = self.parent_id.map(|id| id.to_string());

        IndexMap::from([
            ("id", id),
            ("file_id", Some(self.file_id.to_string())),
            ("parent_id", parent_id),
            ("reg", self.reg.clone()),
            ("cod_ver", self.cod_ver.clone()),
            ("tipo_escrit", self.tipo_escrit.clone()),
            ("ind_sit_esp", self.ind_sit_esp.clone()),
            ("num_rec_anterior", self.num_rec_anterior.clone()),
            ("dt_ini", self.dt_ini.clone()),
            ("dt_fin", self.dt_fin.clone()),
            ("nome", self.nome.clone()),
            ("cnpj", self.cnpj.clone()),
            ("uf", self.uf.clone()),
            ("cod_mun", self.cod_mun.clone()),
            ("suframa", self.suframa.clone()),
            ("ind_nat_pj", self.ind_nat_pj.clone()),
            ("ind_ativ", self.ind_ativ.clone()),
        ])
    }

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
            .bind(&self.reg)
            .bind(&self.cod_ver)
            .bind(&self.tipo_escrit)
            .bind(&self.ind_sit_esp)
            .bind(&self.num_rec_anterior)
            .bind(&self.dt_ini)
            .bind(&self.dt_fin)
            .bind(&self.nome)
            .bind(&self.cnpj)
            .bind(&self.uf)
            .bind(&self.cod_mun)
            .bind(&self.suframa)
            .bind(&self.ind_nat_pj)
            .bind(&self.ind_ativ)
            .execute(&*DB_POOL)
            .await
        })
    }
}
