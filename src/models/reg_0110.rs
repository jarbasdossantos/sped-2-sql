use crate::database::DB_POOL;
use crate::models::traits::Reg;
use crate::models::utils::get_field;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Reg0110 {
    pub parent_id: Option<i64>,
    pub file_id: i64,
    pub reg: Option<String>,
    pub cod_inc_trib: Option<String>,
    pub ind_apro_cred: Option<String>,
    pub cod_tipo_cont: Option<String>,
    pub ind_reg_cum: Option<String>,
}

impl Reg0110 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self {
        Reg0110 {
            parent_id,
            file_id,
            reg: get_field(&fields, 1),
            cod_inc_trib: get_field(&fields, 2),
            ind_apro_cred: get_field(&fields, 3),
            cod_tipo_cont: get_field(&fields, 4),
            ind_reg_cum: get_field(&fields, 5),
        }
    }
}

impl Reg for Reg0110 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_inc_trib.as_deref(),
            self.ind_apro_cred.as_deref(),
            self.cod_tipo_cont.as_deref(),
            self.ind_reg_cum.as_deref(),
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

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            sqlx::query("INSERT INTO reg_0110 (PARENT_ID, FILE_ID, REG, COD_INC_TRIB, IND_APRO_CRED, COD_TIPO_CONT, IND_REG_CUM) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
                .bind(&self.file_id)
                .bind(&self.reg)
                .bind(&self.cod_inc_trib)
                .bind(&self.ind_apro_cred)
                .bind(&self.cod_tipo_cont)
                .bind(&self.ind_reg_cum)
                .execute(&*DB_POOL).await
        })
    }

    fn values(&self) -> IndexMap<&'static str, Option<String>> {
        todo!()
    }
}
