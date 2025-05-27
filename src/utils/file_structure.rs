use crate::create_loader;
use crate::models::traits::Model;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use std::{collections::HashMap, pin::Pin};

type LoadModelFn = Box<
    dyn Fn(
        i32,
        Option<i32>,
    ) -> Pin<
        Box<
            dyn std::future::Future<
                Output=Result<
                    Vec<Box<dyn Model + Send>>,
                    Box<dyn std::error::Error + Send + Sync>,
                >,
            > + Send,
        >,
    >,
>;

pub struct RegistryEntry {
    pub level: u8,
    pub load_model: Option<LoadModelFn>,
}

unsafe impl Send for RegistryEntry {}
unsafe impl Sync for RegistryEntry {}

pub static FILE_STRUCTURE: Lazy<IndexMap<&str, RegistryEntry>> = Lazy::new(|| {
    IndexMap::from([
        (
            "0000",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_0000::Efd0000),
            },
        ),
        (
            "0001",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0001::Efd0001),
            },
        ),
        (
            "0035",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0035::Efd0035),
            },
        ),
        (
            "0100",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0100::Efd0100),
            },
        ),
        (
            "0110",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0110::Efd0110),
            },
        ),
        (
            "0111",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0111::Efd0111),
            },
        ),
        (
            "0120",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0120::Efd0120),
            },
        ),
        // (
        //     "0125",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_0125::Efd0125),
        //     },
        // ),
        (
            "0140",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0140::Efd0140),
            },
        ),
        (
            "0145",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0145::Efd0145),
            },
        ),
        (
            "0150",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0150::Efd0150),
            },
        ),
        (
            "0190",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0190::Efd0190),
            },
        ),
        (
            "0200",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0200::Efd0200),
            },
        ),
        (
            "0205",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_0205::Efd0205),
            },
        ),
        (
            "0206",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_0206::Efd0206),
            },
        ),
        (
            "0208",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_0208::Efd0208),
            },
        ),
        (
            "0400",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0400::Efd0400),
            },
        ),
        (
            "0450",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_0450::Efd0450),
            },
        ),
        (
            "0500",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0500::Efd0500),
            },
        ),
        (
            "0600",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_0600::Efd0600),
            },
        ),
        // (
        //     "0900",
        //     RegistryEntry {
        //         level: 2,
        //         load_model: create_loader!(crate::models::efd::efd_0900::Efd0900),
        //     },
        // ),
        (
            "0990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_0990::Efd0990),
            },
        ),
        (
            "A001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_a001::EfdA001),
            },
        ),
        (
            "A010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_a010::EfdA010),
            },
        ),
        (
            "A100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_a100::EfdA100),
            },
        ),
        (
            "A110",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_a110::EfdA110),
            },
        ),
        (
            "A111",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_a111::EfdA111),
            },
        ),
        (
            "A120",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_a120::EfdA120),
            },
        ),
        (
            "A170",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_a170::EfdA170),
            },
        ),
        (
            "A990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_a990::EfdA990),
            },
        ),
        (
            "C001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_c001::EfdC001),
            },
        ),
        (
            "C010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_c010::EfdC010),
            },
        ),
        (
            "C100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c100::EfdC100),
            },
        ),
        (
            "C110",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c110::EfdC110),
            },
        ),
        (
            "C111",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c111::EfdC111),
            },
        ),
        (
            "C120",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c120::EfdC120),
            },
        ),
        (
            "C170",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c170::EfdC170),
            },
        ),
        (
            "C175",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C180",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c180::EfdC180),
            },
        ),
        (
            "C181",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c181::EfdC181),
            },
        ),
        (
            "C185",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c185::EfdC185),
            },
        ),
        (
            "C188",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c188::EfdC188),
            },
        ),
        (
            "C190",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c190::EfdC190),
            },
        ),
        (
            "C191",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c191::EfdC191),
            },
        ),
        (
            "C195",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c195::EfdC195),
            },
        ),
        (
            "C198",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c198::EfdC198),
            },
        ),
        (
            "C199",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c199::EfdC199),
            },
        ),
        (
            "C380",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c380::EfdC380),
            },
        ),
        (
            "C381",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c381::EfdC381),
            },
        ),
        (
            "C385",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c385::EfdC385),
            },
        ),
        (
            "C395",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c395::EfdC395),
            },
        ),
        (
            "C396",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c396::EfdC396),
            },
        ),
        (
            "C400",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c400::EfdC400),
            },
        ),
        (
            "C405",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c405::EfdC405),
            },
        ),
        (
            "C481",
            RegistryEntry {
                level: 5,
                load_model: create_loader!(crate::models::efd::efd_c481::EfdC481),
            },
        ),
        (
            "C485",
            RegistryEntry {
                level: 5,
                load_model: create_loader!(crate::models::efd::efd_c485::EfdC485),
            },
        ),
        (
            "C489",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c489::EfdC489),
            },
        ),
        (
            "C490",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c490::EfdC490),
            },
        ),
        (
            "C491",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c491::EfdC491),
            },
        ),
        (
            "C495",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c495::EfdC495),
            },
        ),
        (
            "C499",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c499::EfdC499),
            },
        ),
        (
            "C500",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c500::EfdC500),
            },
        ),
        (
            "C501",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c501::EfdC501),
            },
        ),
        (
            "C505",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c505::EfdC505),
            },
        ),
        (
            "C509",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c509::EfdC509),
            },
        ),
        (
            "C600",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_c600::EfdC600),
            },
        ),
        (
            "C601",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c601::EfdC601),
            },
        ),
        (
            "C605",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c605::EfdC605),
            },
        ),
        (
            "C609",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_c609::EfdC609),
            },
        ),
        (
            "C800",
            RegistryEntry {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C810",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C820",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C830",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C860",
            RegistryEntry {
                level: 3,
                load_model: None,
            },
        ),
        (
            "C870",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C880",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C890",
            RegistryEntry {
                level: 4,
                load_model: None,
            },
        ),
        (
            "C990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_c990::EfdC990),
            },
        ),
        (
            "D001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_d001::EfdD001),
            },
        ),
        (
            "D010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_d010::EfdD010),
            },
        ),
        (
            "D100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d100::EfdD100),
            },
        ),
        (
            "D101",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d101::EfdD101),
            },
        ),
        (
            "D105",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d105::EfdD105),
            },
        ),
        (
            "D111",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d111::EfdD111),
            },
        ),
        (
            "D200",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d200::EfdD200),
            },
        ),
        (
            "D201",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d201::EfdD201),
            },
        ),
        (
            "D205",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d205::EfdD205),
            },
        ),
        (
            "D209",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d209::EfdD209),
            },
        ),
        (
            "D300",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d300::EfdD300),
            },
        ),
        (
            "D309",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d309::EfdD309),
            },
        ),
        (
            "D350",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d350::EfdD350),
            },
        ),
        (
            "D359",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d359::EfdD359),
            },
        ),
        (
            "D500",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d500::EfdD500),
            },
        ),
        (
            "D501",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d501::EfdD501),
            },
        ),
        (
            "D505",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d505::EfdD505),
            },
        ),
        (
            "D509",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d509::EfdD509),
            },
        ),
        (
            "D600",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_d600::EfdD600),
            },
        ),
        (
            "D601",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d601::EfdD601),
            },
        ),
        (
            "D605",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d605::EfdD605),
            },
        ),
        (
            "D609",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_d609::EfdD609),
            },
        ),
        (
            "D990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_d990::EfdD990),
            },
        ),
        (
            "F001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_f001::EfdF001),
            },
        ),
        (
            "F010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_f010::EfdF010),
            },
        ),
        (
            "F100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f100::EfdF100),
            },
        ),
        (
            "F111",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f111::EfdF111),
            },
        ),
        (
            "F120",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f120::EfdF120),
            },
        ),
        (
            "F129",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f129::EfdF129),
            },
        ),
        (
            "F130",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f130::EfdF130),
            },
        ),
        (
            "F139",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f139::EfdF139),
            },
        ),
        (
            "F150",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f150::EfdF150),
            },
        ),
        (
            "F200",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f200::EfdF200),
            },
        ),
        (
            "F205",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f205::EfdF205),
            },
        ),
        (
            "F210",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f210::EfdF210),
            },
        ),
        (
            "F211",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f211::EfdF211),
            },
        ),
        (
            "F500",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f500::EfdF500),
            },
        ),
        (
            "F509",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f509::EfdF509),
            },
        ),
        (
            "F510",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f510::EfdF510),
            },
        ),
        (
            "F519",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f519::EfdF519),
            },
        ),
        (
            "F525",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f525::EfdF525),
            },
        ),
        (
            "F550",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f550::EfdF550),
            },
        ),
        (
            "F559",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f559::EfdF559),
            },
        ),
        (
            "F560",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f560::EfdF560),
            },
        ),
        (
            "F569",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_f569::EfdF569),
            },
        ),
        (
            "F600",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f600::EfdF600),
            },
        ),
        (
            "F700",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f700::EfdF700),
            },
        ),
        (
            "F800",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_f800::EfdF800),
            },
        ),
        (
            "F990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_f990::EfdF990),
            },
        ),
        (
            "I001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_i001::EfdI001),
            },
        ),
        (
            "I010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_i010::EfdI010),
            },
        ),
        (
            "I100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_i100::EfdI100),
            },
        ),
        (
            "I199",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_i199::EfdI199),
            },
        ),
        (
            "I200",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_i200::EfdI200),
            },
        ),
        (
            "I299",
            RegistryEntry {
                level: 5,
                load_model: create_loader!(crate::models::efd::efd_i299::EfdI299),
            },
        ),
        (
            "I300",
            RegistryEntry {
                level: 5,
                load_model: create_loader!(crate::models::efd::efd_i300::EfdI300),
            },
        ),
        (
            "I399",
            RegistryEntry {
                level: 6,
                load_model: create_loader!(crate::models::efd::efd_i399::EfdI399),
            },
        ),
        (
            "I990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_i990::EfdI990),
            },
        ),
        (
            "M001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_m001::EfdM001),
            },
        ),
        (
            "M100",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m100::EfdM100),
            },
        ),
        (
            "M105",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m105::EfdM105),
            },
        ),
        (
            "M110",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m110::EfdM110),
            },
        ),
        // (
        //     "M115",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_m115::EfdM115),
        //     },
        // ),
        (
            "M200",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m200::EfdM200),
            },
        ),
        (
            "M205",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m205::EfdM205),
            },
        ),
        (
            "M210",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m210::EfdM210),
            },
        ),
        (
            "M211",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_m211::EfdM211),
            },
        ),
        // (
        //     "M215",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_m215::EfdM215),
        //     },
        // ),
        (
            "M220",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_m220::EfdM220),
            },
        ),
        // (
        //     "M225",
        //     RegistryEntry {
        //         level: 5,
        //         load_model: create_loader!(crate::models::efd::efd_m225::EfdM225),
        //     },
        // ),
        (
            "M230",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_m230::EfdM230),
            },
        ),
        (
            "M300",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m300::EfdM300),
            },
        ),
        (
            "M350",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m350::EfdM350),
            },
        ),
        (
            "M400",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m400::EfdM400),
            },
        ),
        (
            "M410",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m410::EfdM410),
            },
        ),
        (
            "M500",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m500::EfdM500),
            },
        ),
        (
            "M505",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m505::EfdM505),
            },
        ),
        (
            "M510",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m510::EfdM510),
            },
        ),
        // (
        //     "M515",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_m515::EfdM515),
        //     },
        // ),
        (
            "M600",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m600::EfdM600),
            },
        ),
        (
            "M605",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m605::EfdM605),
            },
        ),
        (
            "M610",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m610::EfdM610),
            },
        ),
        (
            "M611",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_m611::EfdM611),
            },
        ),
        // (
        //     "M615",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_m615::EfdM615),
        //     },
        // ),
        (
            "M620",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_m620::EfdM620),
            },
        ),
        // (
        //     "M625",
        //     RegistryEntry {
        //         level: 5,
        //         load_model: create_loader!(crate::models::efd::efd_m625::EfdM625),
        //     },
        // ),
        (
            "M630",
            RegistryEntry {
                level: 5,
                load_model: create_loader!(crate::models::efd::efd_m630::EfdM630),
            },
        ),
        (
            "M700",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m700::EfdM700),
            },
        ),
        (
            "M700",
            RegistryEntry {
                level: 2,
                load_model: None,
            },
        ),
        (
            "M800",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_m800::EfdM800),
            },
        ),
        (
            "M810",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_m810::EfdM810),
            },
        ),
        (
            "M990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_m990::EfdM990),
            },
        ),
        (
            "P001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_p001::EfdP001),
            },
        ),
        (
            "P010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_p010::EfdP010),
            },
        ),
        (
            "P100",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_p100::EfdP100),
            },
        ),
        (
            "P110",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_p110::EfdP110),
            },
        ),
        (
            "P199",
            RegistryEntry {
                level: 4,
                load_model: create_loader!(crate::models::efd::efd_p199::EfdP199),
            },
        ),
        (
            "P200",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_p200::EfdP200),
            },
        ),
        (
            "P210",
            RegistryEntry {
                level: 3,
                load_model: create_loader!(crate::models::efd::efd_p210::EfdP210),
            },
        ),
        (
            "P990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_p990::EfdP990),
            },
        ),
        (
            "1001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_1001::Efd1001),
            },
        ),
        (
            "1010",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1010::Efd1010),
            },
        ),
        // (
        //     "1011",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1011::Efd1011),
        //     },
        // ),
        (
            "1020",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1020::Efd1020),
            },
        ),
        // (
        //     "1050",
        //     RegistryEntry {
        //         level: 2,
        //         load_model: create_loader!(crate::models::efd::efd_1050::Efd1050),
        //     },
        // ),
        (
            "1100",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1100::Efd1100),
            },
        ),
        // (
        //     "1101",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1101::Efd1101),
        //     },
        // ),
        // (
        //     "1102",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_1102::Efd1102),
        //     },
        // ),
        // (
        //     "1200",
        //     RegistryEntry {
        //         level: 2,
        //         load_model: create_loader!(crate::models::efd::efd_1200::Efd1200),
        //     },
        // ),
        // (
        //     "1210",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1210::Efd1210),
        //     },
        // ),
        // (
        //     "1220",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1220::Efd1220),
        //     },
        // ),
        (
            "1300",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1300::Efd1300),
            },
        ),
        (
            "1500",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1500::Efd1500),
            },
        ),
        // (
        //     "1501",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1501::Efd1501),
        //     },
        // ),
        // (
        //     "1502",
        //     RegistryEntry {
        //         level: 4,
        //         load_model: create_loader!(crate::models::efd::efd_1502::Efd1502),
        //     },
        // ),
        // (
        //     "1600",
        //     RegistryEntry {
        //         level: 2,
        //         load_model: create_loader!(crate::models::efd::efd_1600::Efd1600),
        //     },
        // ),
        // (
        //     "1610",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1610::Efd1610),
        //     },
        // ),
        // (
        //     "1620",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1620::Efd1620),
        //     },
        // ),
        (
            "1700",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1700::Efd1700),
            },
        ),
        (
            "1800",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1800::Efd1800),
            },
        ),
        // (
        //     "1809",
        //     RegistryEntry {
        //         level: 3,
        //         load_model: create_loader!(crate::models::efd::efd_1809::Efd1809),
        //     },
        // ),
        (
            "1900",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_1900::Efd1900),
            },
        ),
        (
            "1990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_1990::Efd1990),
            },
        ),
        (
            "9001",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_9001::Efd9001),
            },
        ),
        (
            "9900",
            RegistryEntry {
                level: 2,
                load_model: create_loader!(crate::models::efd::efd_9900::Efd9900),
            },
        ),
        (
            "9990",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_9990::Efd9990),
            },
        ),
        (
            "9999",
            RegistryEntry {
                level: 1,
                load_model: create_loader!(crate::models::efd::efd_9999::Efd9999),
            },
        ),
    ])
});

pub fn get_reg_children() -> HashMap<&'static str, Vec<&'static str>> {
    let mut children: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut parent_stack: Vec<(&'static str, u8)> = Vec::new(); // Stack of (reg_code, level)

    for (reg, structure) in FILE_STRUCTURE.iter() {
        while let Some(&(_, parent_level)) = parent_stack.last() {
            if parent_level >= structure.level {
                parent_stack.pop();
            } else {
                break;
            }
        }

        if let Some(&(parent_reg, _)) = parent_stack.last() {
            if structure.level == parent_stack.last().unwrap().1 + 1 {
                children
                    .entry(parent_reg)
                    .or_insert_with(Vec::new)
                    .push(reg);
            }
        }

        parent_stack.push((reg, structure.level));
    }

    children
}
