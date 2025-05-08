use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use crate::utils::database;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

static DB_FIELDS: &'static [&'static str] =
    &["ID", "FILE_ID", "PARENT_ID", "REG", "COD_INF", "TXT_COMPL"];
static TABLE: &str = "reg_C110";

#[derive(Debug)]
pub struct RegC110 {
    pub id: Option<i64>,
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_inf: Option<String>,
    pub txt_compl: Option<String>,
}

impl RegC110 {
    pub fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegC110 {
            id,
            file_id,
            parent_id,
            reg: get_field(&fields, 1),
            cod_inf: get_field(&fields, 2),
            txt_compl: get_field(&fields, 3),
        }
    }
}

impl Reg for RegC110 {
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
            .bind(&self.cod_inf.as_deref())
            .bind(&self.txt_compl.as_deref())
            .execute(&*DB_POOL)
            .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        todo!()
    }
}
