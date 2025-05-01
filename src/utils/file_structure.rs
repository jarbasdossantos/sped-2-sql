use std::collections::HashMap;

use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::models::{
    reg_0000::Reg0000, reg_0001::Reg0001, reg_0140::Reg0140, reg_0150::Reg0150, reg_0190::Reg0190, reg_0200::Reg0200, reg_0500::Reg0500, traits::{Model, Reg}
};

#[derive(Debug)]
pub struct Struct {
    pub level: u8,
    pub load_model:
        Option<fn(i64, Option<i64>) -> Result<Vec<Box<dyn Reg>>, Box<dyn std::error::Error>>>,
}

impl Struct {}

pub static FILE_STRUCTURE: Lazy<IndexMap<&str, Struct>> = Lazy::new(|| {
    IndexMap::from([
        (
            "0000",
            Struct {
                level: 0,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0000::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0001",
            Struct {
                level: 1,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0001::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0035",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0100",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0110",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0111",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "0120",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0125",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "0140",
            Struct {
                level: 2,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0140::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0145",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "0150",
            Struct {
                level: 3,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0150::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0190",
            Struct {
                level: 3,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0190::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0200",
            Struct {
                level: 3,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0200::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0205",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "0206",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "0208",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "0400",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "0450",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "0500",
            Struct {
                level: 2,
                load_model: Some(|file_id, parent_id| {
                    Ok(Reg0500::load(file_id, parent_id)?
                        .into_iter()
                        .map(|reg| Box::new(reg) as Box<dyn Reg>)
                        .collect())
                }),
            },
        ),
        (
            "0600",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0900",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "0990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "A001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "A010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "A100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "A110",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "A111",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "A120",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "A170",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "A990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "C001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "C010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "C100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C110",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C111",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C120",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C170",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C175",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C180",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C181",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C185",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C188",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C190",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C191",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C195",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C198",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C199",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C380",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C381",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C385",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C395",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C396",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C400",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C405",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C481",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "C485",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "C489",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C490",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C491",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C495",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C499",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C500",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C501",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C505",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C509",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C600",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C601",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C605",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C609",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C800",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C810",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C820",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C830",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C860",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C870",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C880",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C890",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "D001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "D010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "D100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D101",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D105",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D111",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D200",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D201",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D205",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D209",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D300",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D309",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D350",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D359",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D500",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D501",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D505",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D509",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D600",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "D601",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D605",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D609",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "D990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "F001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "F010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "F100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F111",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F120",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F129",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F130",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F139",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F150",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F200",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F205",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F210",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F211",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F500",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F509",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F510",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F519",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F525",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F550",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F559",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F560",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F569",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "F600",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F700",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F800",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "F990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "I001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "I010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "I100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "I199",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "I200",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "I299",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "I300",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "I399",
            Struct {
                level: 6,
                load_model: None,
            },
        ),
        (
            "I990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "M001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "M100",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M105",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M110",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M115",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M200",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M205",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M210",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M211",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M215",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M220",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M225",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "M230",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M300",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M350",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M400",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M410",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M500",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M505",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M510",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M515",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M600",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M605",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M610",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M611",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M615",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M620",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M625",
            Struct {
                level: 5,
                load_model: None,
            },
        ),
        (
            "M630",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "M700",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M800",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M810",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "M990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "P001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "P010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "P100",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "P110",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "P199",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "P200",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "P210",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "P990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "1001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "1010",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1011",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1020",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1050",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1100",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1101",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1102",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "1200",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1210",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1220",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1300",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1500",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1501",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1502",
            Struct {
                level: 4,
                load_model: None,
            },
        ),
        (
            "1600",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1610",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1620",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1700",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1800",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1809",
            Struct {
                level: 3,
                load_model: None,
            },
        ),
        (
            "1900",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "1990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "9001",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "9900",
            Struct {
                level: 2,
                load_model: None,
            },
        ),
        (
            "9990",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
        (
            "9999",
            Struct {
                level: 1,
                load_model: None,
            },
        ),
    ])
});

pub fn get_reg_children() -> HashMap<&'static str, Vec<&'static str>> {
    let mut children: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut parent_stack: Vec<(&'static str, u8)> = Vec::new(); // Stack of (reg_code, level)

    for (reg, structure) in FILE_STRUCTURE.iter() {
        // Pop parents from the stack whose level is greater than or equal to the current level
        while let Some(&(_, parent_level)) = parent_stack.last() {
            if parent_level >= structure.level {
                parent_stack.pop();
            } else {
                break; // Found the correct parent level
            }
        }

        // If there's a parent on the stack, the current reg is its child
        if let Some(&(parent_reg, _)) = parent_stack.last() {
            // Ensure the child's level is exactly one greater than the parent's level
            if structure.level == parent_stack.last().unwrap().1 + 1 {
                children
                    .entry(parent_reg)
                    .or_insert_with(Vec::new)
                    .push(reg);
            } else {
                // This case might indicate an issue in the FILE_STRUCTURE definition
                // or a need for more complex hierarchy logic (e.g., skipping levels).
                // For now, we only add direct children (level + 1).
                // You might want to handle this differently based on requirements.
                // eprintln!("Warning: Register {} level {} is not a direct child of {}", reg, structure.level, parent_reg);
            }
        } else if structure.level > 0 {
            // This handles cases where a register > level 0 appears without a preceding parent
            // in the defined order. This might indicate an issue in FILE_STRUCTURE order.
            // eprintln!("Warning: Register {} level {} found without a parent on the stack.", reg, structure.level);
        }

        // Push the current register onto the stack as a potential parent
        parent_stack.push((reg, structure.level));
    }

    // The function now returns the map instead of just printing it
    children
}
