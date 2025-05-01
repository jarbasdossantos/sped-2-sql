use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use futures::executor::block_on;
use indexmap::IndexMap;
use sqlx::Row;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "DT_ALT",
    "COD_NAT_CC",
    "IND_CTA",
    "NIVEL",
    "COD_CTA",
    "NOME_CTA",
    "COD_CTA_REF",
    "CNPJ_EST",
];
static TABLE: &str = "reg_0500";

#[derive(Debug)]
pub struct Reg0500 {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub dt_alt: Option<String>,
    pub cod_nat_cc: Option<String>,
    pub ind_cta: Option<String>,
    pub nivel: Option<String>,
    pub cod_cta: Option<String>,
    pub nome_cta: Option<String>,
    pub cod_cta_ref: Option<String>,
    pub cnpj_est: Option<String>,
}

impl Model for Reg0500 {
    fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0500 {
            id: fields.get(0).and_then(|v| v.parse().ok()),
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            dt_alt: get_field(&fields, 2),
            cod_nat_cc: get_field(&fields, 3),
            ind_cta: get_field(&fields, 4),
            nivel: get_field(&fields, 5),
            cod_cta: get_field(&fields, 6),
            nome_cta: get_field(&fields, 7),
            cod_cta_ref: get_field(&fields, 8),
            cnpj_est: get_field(&fields, 9),
        }
    }

    fn load(file_id: i64, parent_id: Option<i64>) -> anyhow::Result<Vec<Self>, anyhow::Error> {
        block_on(async move {
            let rows = sqlx::query(
                format!(
                    "SELECT {} FROM {} WHERE FILE_ID = ? AND PARENT_ID = ?",
                    DB_FIELDS.join(", "),
                    TABLE
                )
                .as_str(),
            )
            .bind(file_id)
            .bind(parent_id)
            .fetch_all(&*DB_POOL)
            .await?;

            let mut data = Vec::new();

            for row in rows {
                let parent_id = row.try_get::<i64, _>("PARENT_ID").unwrap_or(0).into();

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

                data.push(Self::new(fields, parent_id, file_id));
            }

            Ok(data)
        })
    }
}

impl Reg for Reg0500 {
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
                    "INSERT INTO {TABLE} ({}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    DB_FIELDS.join(", ")
                )
                .as_str(),
            )
            .bind(&self.parent_id)
            .bind(&self.file_id)
            .bind(&self.reg)
            .bind(&self.dt_alt)
            .bind(&self.cod_nat_cc)
            .bind(&self.ind_cta)
            .bind(&self.nivel)
            .bind(&self.cod_cta)
            .bind(&self.nome_cta)
            .bind(&self.cod_cta_ref)
            .bind(&self.cnpj_est)
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
            ("dt_alt", self.dt_alt.clone()),
            ("cod_nat_cc", self.cod_nat_cc.clone()),
            ("ind_cta", self.ind_cta.clone()),
            ("nivel", self.nivel.clone()),
            ("cod_cta", self.cod_cta.clone()),
            ("nome_cta", self.nome_cta.clone()),
            ("cod_cta_ref", self.cod_cta_ref.clone()),
            ("cnpj_est", self.cnpj_est.clone()),
        ])
    }
}
