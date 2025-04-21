use crate::models::reg_trait::Reg;
use crate::models::utils::get_field;
use sqlx::SqlitePool;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Reg0200 {
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

impl Reg0200 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>) -> Self {
        Reg0200 {
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
}

impl Reg for Reg0200 {
    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_item.as_deref(),
            self.descr_item.as_deref(),
            self.cod_barra.as_deref(),
            self.cod_ant_item.as_deref(),
            self.unid_inv.as_deref(),
            self.tipo_item.as_deref(),
            self.cod_ncm.as_deref(),
            self.ex_ipi.as_deref(),
            self.cod_gen.as_deref(),
            self.cod_lst.as_deref(),
            self.aliq_icms.as_deref(),
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
            sqlx::query("INSERT INTO reg_0200 (PARENT_ID, REG, COD_ITEM, DESCR_ITEM, COD_BARRA, COD_ANT_ITEM, UNID_INV, TIPO_ITEM, COD_NCM, EX_IPI, COD_GEN, COD_LST, ALIQ_ICMS) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
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
                .execute(conn).await
        })
    }
}
