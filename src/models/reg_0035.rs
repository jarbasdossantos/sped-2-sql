use sqlx::FromRow;

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0035 {
    pub reg: Option<String>,
    pub cod_scp: Option<String>,
    pub nome_scp: Option<String>,
    pub inf_comp: Option<String>,
}

impl Reg for Reg0035 {
    fn new(fields: Vec<&str>) -> Self {
        Reg0035 {
            reg: get_field(&fields, 1),
            cod_scp: get_field(&fields, 2),
            nome_scp: get_field(&fields, 3),
            inf_comp: get_field(&fields, 4),
        }
    }

    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_scp.as_deref(),
            self.nome_scp.as_deref(),
            self.inf_comp.as_deref(),
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
