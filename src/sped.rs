use crate::models::reg_0000::Reg0000;
use crate::models::reg_0035::Reg0035;
use crate::models::reg_0150::Reg0150;
use crate::models::reg_trait::Reg;
use lazy_static::lazy_static;
use std::collections::HashMap;

type RegFactory = fn(Vec<&str>) -> Box<dyn Reg>;

// Create a static map of registers and their corresponding factory functions
lazy_static! {
    static ref REG_FACTORIES: HashMap<&'static str, RegFactory> = {
        let mut map = HashMap::<&'static str, RegFactory>::new();

        map.insert("0000", |fields| {
            Box::new(Reg0000::new(fields)) as Box<dyn Reg>
        });

        map.insert("0035", |fields| {
            Box::new(Reg0035::new(fields)) as Box<dyn Reg>
        });

        map.insert("0150", |fields| {
            Box::new(Reg0150::new(fields)) as Box<dyn Reg>
        });

        map
    };
}

// Handle each line of the file
pub fn handle_line(line: &str) {
    let data = line.split("|").collect::<Vec<&str>>();

    if let Some(factory) = REG_FACTORIES.get(data.get(1).expect("No register code found")) {
        let _reg: Box<dyn Reg> = factory(data);
    } else {
        println!(
            "Register not found in map: {}",
            data.get(1).expect("No register code found")
        );
    }
}
