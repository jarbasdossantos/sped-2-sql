use indexmap::IndexMap;
use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct RegC110 {
    pub file_id: i64,
    pub parent_id: Option<i64>,
    pub cod_inf: Option<String>,
    pub txt_compl: Option<String>,
}

impl RegC110 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        RegC110 {
            file_id,
            parent_id,
            cod_inf: get_field(&fields, 2),
            txt_compl: get_field(&fields, 3),
        }
    }
}

impl Reg for RegC110 {
    fn to_line(&self) -> String {
        format!(
            "C110|{}|{}",
            self.cod_inf.as_deref().unwrap_or(""),
            self.txt_compl.as_deref().unwrap_or("")
        )
    }

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query(
                "INSERT INTO reg_c110 (FILE_ID, PARENT_ID, REG, COD_INF, TXT_COMPL) VALUES (?, ?, ?, ?, ?)",
            )
                .bind(self.file_id)
                .bind(self.parent_id)
                .bind("C110")
                .bind(self.cod_inf.as_deref())
                .bind(self.txt_compl.as_deref())
                .execute(&*DB_POOL)
                .await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        todo!()
    }
}
