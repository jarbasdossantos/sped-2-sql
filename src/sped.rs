use crate::models::reg_0000::Reg0000;
use crate::models::reg_0001::Reg0001;
use crate::models::reg_0035::Reg0035;
use crate::models::reg_0100::Reg0100;
use crate::models::reg_0110::Reg0110;
use crate::models::reg_0140::Reg0140;
use crate::models::reg_0150::Reg0150;
use crate::models::reg_0190::Reg0190;
use crate::models::reg_0200::Reg0200;
use crate::models::reg_9999::Reg9999;
use crate::models::reg_c001::RegC001;
use crate::models::reg_c010::RegC010;
use crate::models::reg_c100::RegC100;
use crate::models::reg_c110::RegC110;
use crate::models::reg_c180::RegC180;
use crate::models::reg_c181::RegC181;
use crate::models::reg_c185::RegC185;
use crate::models::reg_p200::RegP200;
use crate::models::traits::{Model, Reg};
use anyhow::Result;

pub fn factories(
    reg: &str,
    parent_id: Option<i64>,
    file_id: i64,
    fields: Vec<&str>,
) -> Option<Box<dyn Reg>> {
    match reg {
        "0000" => Some(Box::new(Reg0000::new(fields, None, parent_id, file_id))),
        "0001" => Some(Box::new(Reg0001::new(fields, None, parent_id, file_id))),
        "0035" => Some(Box::new(Reg0035::new(fields, None, parent_id, file_id))),
        "0100" => Some(Box::new(Reg0100::new(fields, None, parent_id, file_id))),
        "0110" => Some(Box::new(Reg0110::new(fields, None, parent_id, file_id))),
        "0140" => Some(Box::new(Reg0140::new(fields, None, parent_id, file_id))),
        "0150" => Some(Box::new(Reg0150::new(fields, None, parent_id, file_id))),
        "0190" => Some(Box::new(Reg0190::new(fields, None, parent_id, file_id))),
        "0200" => Some(Box::new(Reg0200::new(fields, None, parent_id, file_id))),
        "C001" => Some(Box::new(RegC001::new(fields, None, parent_id, file_id))),
        "C010" => Some(Box::new(RegC010::new(fields, None, parent_id, file_id))),
        "C100" => Some(Box::new(RegC100::new(fields, None, parent_id, file_id))),
        "C110" => Some(Box::new(RegC110::new(fields, None, parent_id, file_id))),
        "C180" => Some(Box::new(RegC180::new(fields, None, parent_id, file_id))),
        "C181" => Some(Box::new(RegC181::new(fields, None, parent_id, file_id))),
        "C185" => Some(Box::new(RegC185::new(fields, None, parent_id, file_id))),
        "P200" => Some(Box::new(RegP200::new(fields, None, parent_id, file_id))),
        "9999" => Some(Box::new(Reg9999::new(fields, None, parent_id, file_id))),
        _ => None,
    }
}

pub async fn handle_line(
    line: &str,
    parent_id: i64,
    file_id: i64,
) -> Result<Option<sqlx::sqlite::SqliteQueryResult>, sqlx::Error> {
    let data = line.split("|").collect::<Vec<&str>>();
    let reg_code = data.get(1).unwrap_or(&"");

    if let Some(factory) = factories(reg_code, Some(parent_id), file_id, data) {
        let reg: Box<dyn Reg> = factory;

        match reg.save().await {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(error),
        }
    } else {
        Ok(None)
    }
}
