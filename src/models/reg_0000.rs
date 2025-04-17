use std::future::Future;
use std::pin::Pin;
use sqlx::SqlitePool;
use crate::models::reg_trait::Reg;
use crate::models::utils::{get_date, get_field};

#[derive(Debug)]
pub struct Reg0000 {
    pub parent_id: Option<i64>,
    pub reg: Option<String>,
    pub cod_ver: Option<String>,
    pub tipo_escrit: Option<String>,
    pub ind_sit_esp: Option<String>,
    pub num_rec_anterior: Option<String>,
    pub dt_ini: Option<chrono::NaiveDate>,
    pub dt_fin: Option<chrono::NaiveDate>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub uf: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub ind_nat_pj: Option<String>,
    pub ind_ativ: Option<String>,
}

impl Reg0000 {
    pub fn new(fields: Vec<&str>, parent_id: Option<i64>) -> Self {
        Reg0000 {
            parent_id,
            reg: get_field(&fields, 1),
            cod_ver: get_field(&fields, 2),
            tipo_escrit: get_field(&fields, 3),
            ind_sit_esp: get_field(&fields, 4),
            num_rec_anterior: get_field(&fields, 5),
            dt_ini: get_date(&fields, 6),
            dt_fin: get_date(&fields, 7),
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
    fn to_line(&self) -> String {
        let dt_ini = self.dt_ini.map(|d| d.format("%d%m%Y").to_string());
        let dt_fin = self.dt_fin.map(|d| d.format("%d%m%Y").to_string());

        let fields = [
            self.reg.as_deref(),
            self.cod_ver.as_deref(),
            self.tipo_escrit.as_deref(),
            self.ind_sit_esp.as_deref(),
            self.num_rec_anterior.as_deref(),
            dt_ini.as_deref(),
            dt_fin.as_deref(),
            self.nome.as_deref(),
            self.cnpj.as_deref(),
            self.uf.as_deref(),
            self.cod_mun.as_deref(),
            self.suframa.as_deref(),
            self.ind_nat_pj.as_deref(),
            self.ind_ativ.as_deref(),
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
            sqlx::query("\
            INSERT INTO reg_0000\
                (parent_id, reg, cod_ver, tipo_escrit, ind_sit_esp, num_rec_anterior, dt_ini, dt_fin, nome, cnpj, uf, cod_mun, suframa, ind_nat_pj, ind_ativ)\
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&self.parent_id)
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
                .execute(conn).await
        })
    }
}
