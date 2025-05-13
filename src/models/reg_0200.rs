use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use indexmap::IndexMap;
use sqlx::Row;
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

use super::traits::Model;

static DB_FIELDS: &'static [&'static str] = &[
    "ID",
    "FILE_ID",
    "PARENT_ID",
    "REG",
    "COD_ITEM",
    "DESCR_ITEM",
    "COD_BARRA",
    "COD_ANT_ITEM",
    "UNID_INV",
    "TIPO_ITEM",
    "COD_NCM",
    "EX_IPI",
    "COD_GEN",
    "COD_LST",
    "ALIQ_ICMS",
];
static TABLE: &str = "reg_0200";

#[derive(Debug)]
pub struct Reg0200 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_item: Option<String>,
    pub descr_item: Option<String>,
    pub cod_barra: Option<String>,
    pub cod_ant_item: Option<String>,
    pub unid_inv: Option<String>,
    pub tipo_item: Option<String>,
    pub cod_ncm: Option<String>,
    pub ex_ipi: Option<String>,
    pub cod_gen: Option<String>,
    pub cod_lst: Option<String>,
    pub aliq_icms: Option<String>,
}

#[async_trait]
impl Model for Reg0200 {
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0200 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            cod_item: get_field(&fields, 2),
            descr_item: get_field(&fields, 3),
            cod_barra: get_field(&fields, 4),
            cod_ant_item: get_field(&fields, 5),
            unid_inv: get_field(&fields, 6),
            tipo_item: get_field(&fields, 7),
            cod_ncm: get_field(&fields, 8),
            ex_ipi: get_field(&fields, 9),
            cod_gen: get_field(&fields, 10),
            cod_lst: get_field(&fields, 11),
            aliq_icms: get_field(&fields, 12),
        }
    }

    async fn load(file_id: i64, parent_id: Option<i64>) -> anyhow::Result<Vec<Self>, anyhow::Error> {
        {
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

                data.push(Self::new(
                    fields[2..].to_vec(),
                    fields.get(0).and_then(|v| v.parse().ok()),
                    parent_id,
                    file_id,
                ));
            }

            Ok(data)
        }
    }
}

impl Reg for Reg0200 {
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
            .bind(&self.cod_item)
            .bind(&self.descr_item)
            .bind(&self.cod_barra)
            .bind(&self.cod_ant_item)
            .bind(&self.unid_inv)
            .bind(&self.tipo_item)
            .bind(&self.cod_ncm)
            .bind(&self.ex_ipi)
            .bind(&self.cod_gen)
            .bind(&self.cod_lst)
            .bind(&self.aliq_icms)
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
            ("cod_item", self.cod_item.clone()),
            ("descr_item", self.descr_item.clone()),
            ("cod_barra", self.cod_barra.clone()),
            ("cod_ant_item", self.cod_ant_item.clone()),
            ("unid_inv", self.unid_inv.clone()),
            ("tipo_item", self.tipo_item.clone()),
            ("cod_ncm", self.cod_ncm.clone()),
            ("ex_ipi", self.ex_ipi.clone()),
            ("cod_gen", self.cod_gen.clone()),
            ("cod_lst", self.cod_lst.clone()),
            ("aliq_icms", self.aliq_icms.clone()),
        ])
    }
}
