use sqlx::FromRow;

use super::reg_trait::Reg;
use super::utils::{get_date, get_field};

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0000 {
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

impl Reg for Reg0000 {
    fn new(fields: Vec<&str>) -> Self {
        Reg0000 {
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
}
