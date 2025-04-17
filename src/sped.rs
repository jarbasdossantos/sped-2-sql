use crate::models::reg_0000::Reg0000;
use crate::models::reg_0001::Reg0001;
use crate::models::reg_0035::Reg0035;
use crate::models::reg_0150::Reg0150;
use crate::models::reg_0200::Reg0200;
use crate::models::reg_c100::Regc100;
use crate::models::reg_trait::Reg;

pub fn factories(reg: &str, parent_id: Option<i64>, fields: Vec<&str>) -> Option<Box<dyn Reg>> {
    match reg {
        "0000" => Some(Box::new(Reg0000::new(fields, parent_id))),
        "0001" => Some(Box::new(Reg0001::new(fields, parent_id))),
        "0035" => Some(Box::new(Reg0035::new(fields, parent_id))),
        "0150" => Some(Box::new(Reg0150::new(fields, parent_id))),
        "0200" => Some(Box::new(Reg0200::new(fields, parent_id))),
        "C100" => Some(Box::new(Regc100::new(fields, parent_id))),
        _ => None,
    }
}

pub async fn handle_line(
    line: &str,
    parent_id: Option<i64>,
    conn: &sqlx::SqlitePool,
) -> Option<Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> {
    let data = line.split("|").collect::<Vec<&str>>();
    let reg_code = data.get(1).unwrap_or(&"");

    if let Some(factory) = factories(reg_code, parent_id, data) {
        let reg: Box<dyn Reg> = factory;

        let insert = reg.to_db(conn).await;

        if let Err(error) = &insert {
            eprintln!("{}", reg.to_line());
            eprintln!("Error: {}\n", error);
        }

        return Some(insert);
    } else {
        // eprintln!("No register factory found");
    }

    None
}
