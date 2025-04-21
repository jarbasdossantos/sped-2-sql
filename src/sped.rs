use crate::models::reg_0000::Reg0000;
use crate::models::reg_0001::Reg0001;
use crate::models::reg_0035::Reg0035;
use crate::models::reg_0150::Reg0150;
use crate::models::reg_0100::Reg0100;
use crate::models::reg_0110::Reg0110;
use crate::models::reg_0200::Reg0200;
use crate::models::reg_c100::Regc100;
use crate::models::reg_trait::Reg;
use anyhow::Result;

pub fn factories(reg: &str, parent_id: Option<i64>, fields: Vec<&str>) -> Option<Box<dyn Reg>> {
    match reg {
        "0000" => Some(Box::new(Reg0000::new(fields, parent_id))),
        "0001" => Some(Box::new(Reg0001::new(fields, parent_id))),
        "0035" => Some(Box::new(Reg0035::new(fields, parent_id))),
        "0100" => Some(Box::new(Reg0100::new(fields, parent_id))),
        "0110" => Some(Box::new(Reg0110::new(fields, parent_id))),
        "0150" => Some(Box::new(Reg0150::new(fields, parent_id))),
        "0200" => Some(Box::new(Reg0200::new(fields, parent_id))),
        "C100" => Some(Box::new(Regc100::new(fields, parent_id))),
        _ => None,
    }
}

pub async fn handle_line(
    line: &str,
    parent_id: i64,
    conn: &sqlx::SqlitePool,
) -> Result<Option<sqlx::sqlite::SqliteQueryResult>, sqlx::Error> {
    let data = line.split("|").collect::<Vec<&str>>();
    let reg_code = data.get(1).unwrap_or(&"");

    if let Some(factory) = factories(reg_code, Some(parent_id), data) {
        let reg: Box<dyn Reg> = factory;

        match reg.to_db(conn).await {
            Ok(result) => Ok(Some(result)),
            Err(error) => { Err(error) }
        }
    } else {
        Ok(None)
    }
}
