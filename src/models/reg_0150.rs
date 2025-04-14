use sqlx::FromRow;

use super::reg_trait::Reg;
use super::utils::get_field;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Reg0150 {
    pub reg: Option<String>,
    pub cod_part: Option<String>,
    pub nome: Option<String>,
    pub cod_pais: Option<String>,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub ie: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub end: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
}

impl Reg for Reg0150 {
    fn new(fields: Vec<&str>) -> Self {
        Reg0150 {
            reg: get_field(&fields, 1),
            cod_part: get_field(&fields, 2),
            nome: get_field(&fields, 3),
            cod_pais: get_field(&fields, 4),
            cnpj: get_field(&fields, 5),
            cpf: get_field(&fields, 6),
            ie: get_field(&fields, 7),
            cod_mun: get_field(&fields, 8),
            suframa: get_field(&fields, 9),
            end: get_field(&fields, 10),
            num: get_field(&fields, 11),
            compl: get_field(&fields, 12),
            bairro: get_field(&fields, 13),
        }
    }

    fn to_line(&self) -> String {
        let fields = [
            self.reg.as_deref(),
            self.cod_part.as_deref(),
            self.nome.as_deref(),
            self.cod_pais.as_deref(),
            self.cnpj.as_deref(),
            self.cpf.as_deref(),
            self.ie.as_deref(),
            self.cod_mun.as_deref(),
            self.suframa.as_deref(),
            self.end.as_deref(),
            self.num.as_deref(),
            self.compl.as_deref(),
            self.bairro.as_deref(),
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

    fn to_db(&self, reg: &Reg0150, conn: &sqlx::SqlitePool) -> Result<(), Error> {
        todo!()
    }
}
