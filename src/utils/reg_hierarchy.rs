use std::collections::HashMap;
use once_cell::sync::Lazy;

pub struct Struct {
    pub level: u8,
}

impl Struct {}

pub static HIERARCHY: Lazy<HashMap<&str, Struct>> = Lazy::new(|| {
    HashMap::from([
        ("0000", Struct { level: 0 }),
        ("0001", Struct { level: 1 }),
        ("0035", Struct { level: 2 }),
        ("0100", Struct { level: 2 }),
        ("0110", Struct { level: 2 }),
        ("0111", Struct { level: 3 }),
        ("0120", Struct { level: 2 }),
        ("0140", Struct { level: 2 }),
        ("0145", Struct { level: 3 }),
        ("0150", Struct { level: 3 }),
        ("0190", Struct { level: 3 }),
        ("0200", Struct { level: 3 }),
        ("0205", Struct { level: 4 }),
        ("0206", Struct { level: 4 }),
        ("0208", Struct { level: 4 }),
        ("0400", Struct { level: 3 }),
        ("0450", Struct { level: 3 }),
        ("0500", Struct { level: 2 }),
        ("0600", Struct { level: 2 }),
        ("0900", Struct { level: 2 }),
        ("0990", Struct { level: 1 }),
        ("A001", Struct { level: 1 }),
        ("A010", Struct { level: 2 }),
        ("A100", Struct { level: 3 }),
        ("A110", Struct { level: 4 }),
        ("A111", Struct { level: 4 }),
        ("A120", Struct { level: 4 }),
        ("A170", Struct { level: 4 }),
        ("A990", Struct { level: 1 }),
        ("C001", Struct { level: 1 }),
        ("C010", Struct { level: 2 }),
        ("C100", Struct { level: 3 }),
        ("C110", Struct { level: 4 }),
        ("C111", Struct { level: 4 }),
        ("C120", Struct { level: 4 }),
        ("C170", Struct { level: 4 }),
        ("C175", Struct { level: 4 }),
        ("C180", Struct { level: 3 }),
        ("C181", Struct { level: 4 }),
        ("C185", Struct { level: 4 }),
        ("C188", Struct { level: 4 }),
        ("C190", Struct { level: 3 }),
        ("C191", Struct { level: 4 }),
        ("C195", Struct { level: 4 }),
        ("C198", Struct { level: 4 }),
        ("C199", Struct { level: 4 }),
        ("C380", Struct { level: 3 }),
        ("C381", Struct { level: 4 }),
        ("C385", Struct { level: 4 }),
        ("C395", Struct { level: 3 }),
        ("C396", Struct { level: 4 }),
        ("C400", Struct { level: 3 }),
        ("C405", Struct { level: 4 }),
        ("C481", Struct { level: 5 }),
        ("C485", Struct { level: 5 }),
        ("C489", Struct { level: 4 }),
        ("C490", Struct { level: 3 }),
        ("C491", Struct { level: 4 }),
        ("C495", Struct { level: 4 }),
        ("C499", Struct { level: 4 }),
        ("C500", Struct { level: 3 }),
        ("C501", Struct { level: 4 }),
        ("C505", Struct { level: 4 }),
        ("C509", Struct { level: 4 }),
        ("C600", Struct { level: 3 }),
        ("C601", Struct { level: 4 }),
        ("C605", Struct { level: 4 }),
        ("C609", Struct { level: 4 }),
        ("C800", Struct { level: 3 }),
        ("C810", Struct { level: 4 }),
        ("C820", Struct { level: 4 }),
        ("C830", Struct { level: 4 }),
        ("C860", Struct { level: 3 }),
        ("C870", Struct { level: 4 }),
        ("C880", Struct { level: 4 }),
        ("C890", Struct { level: 4 }),
        ("C990", Struct { level: 1 }),
        ("D001", Struct { level: 1 }),
        ("D010", Struct { level: 2 }),
        ("D100", Struct { level: 3 }),
        ("D101", Struct { level: 4 }),
        ("D105", Struct { level: 4 }),
        ("D111", Struct { level: 4 }),
        ("D200", Struct { level: 3 }),
        ("D201", Struct { level: 4 }),
        ("D205", Struct { level: 4 }),
        ("D209", Struct { level: 4 }),
        ("D300", Struct { level: 3 }),
        ("D309", Struct { level: 4 }),
        ("D350", Struct { level: 3 }),
        ("D359", Struct { level: 4 }),
        ("D500", Struct { level: 3 }),
        ("D501", Struct { level: 4 }),
        ("D505", Struct { level: 4 }),
        ("D509", Struct { level: 4 }),
        ("D600", Struct { level: 3 }),
        ("D601", Struct { level: 4 }),
        ("D605", Struct { level: 4 }),
        ("D609", Struct { level: 4 }),
        ("D990", Struct { level: 1 }),
        ("F001", Struct { level: 1 }),
        ("F010", Struct { level: 2 }),
        ("F100", Struct { level: 3 }),
        ("F111", Struct { level: 4 }),
        ("F120", Struct { level: 3 }),
        ("F129", Struct { level: 4 }),
        ("F130", Struct { level: 3 }),
        ("F139", Struct { level: 4 }),
        ("F150", Struct { level: 3 }),
        ("F200", Struct { level: 3 }),
        ("F205", Struct { level: 4 }),
        ("F210", Struct { level: 4 }),
        ("F211", Struct { level: 4 }),
        ("F500", Struct { level: 3 }),
        ("F509", Struct { level: 4 }),
        ("F510", Struct { level: 3 }),
        ("F519", Struct { level: 4 }),
        ("F525", Struct { level: 3 }),
        ("F550", Struct { level: 3 }),
        ("F559", Struct { level: 4 }),
        ("F560", Struct { level: 3 }),
        ("F569", Struct { level: 4 }),
        ("F600", Struct { level: 3 }),
        ("F700", Struct { level: 3 }),
        ("F800", Struct { level: 3 }),
        ("F990", Struct { level: 1 }),
        ("I001", Struct { level: 1 }),
        ("I010", Struct { level: 2 }),
        ("I100", Struct { level: 3 }),
        ("I199", Struct { level: 4 }),
        ("I200", Struct { level: 4 }),
        ("I299", Struct { level: 5 }),
        ("I300", Struct { level: 5 }),
        ("I399", Struct { level: 6 }),
        ("I990", Struct { level: 1 }),
        ("M001", Struct { level: 1 }),
        ("M100", Struct { level: 2 }),
        ("M105", Struct { level: 3 }),
        ("M110", Struct { level: 3 }),
        ("M115", Struct { level: 4 }),
        ("M200", Struct { level: 2 }),
        ("M205", Struct { level: 3 }),
        ("M210", Struct { level: 3 }),
        ("M211", Struct { level: 4 }),
        ("M215", Struct { level: 4 }),
        ("M220", Struct { level: 4 }),
        ("M225", Struct { level: 5 }),
        ("M230", Struct { level: 4 }),
        ("M300", Struct { level: 2 }),
        ("M350", Struct { level: 2 }),
        ("M400", Struct { level: 2 }),
        ("M410", Struct { level: 3 }),
        ("M500", Struct { level: 2 }),
        ("M505", Struct { level: 3 }),
        ("M510", Struct { level: 3 }),
        ("M515", Struct { level: 4 }),
        ("M600", Struct { level: 2 }),
        ("M605", Struct { level: 3 }),
        ("M610", Struct { level: 3 }),
        ("M611", Struct { level: 4 }),
        ("M615", Struct { level: 4 }),
        ("M620", Struct { level: 4 }),
        ("M625", Struct { level: 5 }),
        ("M630", Struct { level: 4 }),
        ("M700", Struct { level: 2 }),
        ("M800", Struct { level: 2 }),
        ("M810", Struct { level: 3 }),
        ("M990", Struct { level: 1 }),
        ("P001", Struct { level: 1 }),
        ("P010", Struct { level: 2 }),
        ("P100", Struct { level: 3 }),
        ("P110", Struct { level: 4 }),
        ("P199", Struct { level: 4 }),
        ("P200", Struct { level: 2 }),
        ("P210", Struct { level: 3 }),
        ("P990", Struct { level: 1 }),
        ("1001", Struct { level: 1 }),
        ("1010", Struct { level: 2 }),
        ("1011", Struct { level: 3 }),
        ("1020", Struct { level: 2 }),
        ("1050", Struct { level: 2 }),
        ("1100", Struct { level: 2 }),
        ("1101", Struct { level: 3 }),
        ("1102", Struct { level: 4 }),
        ("1200", Struct { level: 2 }),
        ("1210", Struct { level: 3 }),
        ("1220", Struct { level: 3 }),
        ("1300", Struct { level: 2 }),
        ("1500", Struct { level: 2 }),
        ("1501", Struct { level: 3 }),
        ("1502", Struct { level: 4 }),
        ("1600", Struct { level: 2 }),
        ("1610", Struct { level: 3 }),
        ("1620", Struct { level: 3 }),
        ("1700", Struct { level: 2 }),
        ("1800", Struct { level: 2 }),
        ("1809", Struct { level: 3 }),
        ("1900", Struct { level: 2 }),
        ("1990", Struct { level: 1 }),
        ("9001", Struct { level: 1 }),
        ("9900", Struct { level: 2 }),
        ("9990", Struct { level: 1 }),
        ("9999", Struct { level: 1 }),
    ])
});