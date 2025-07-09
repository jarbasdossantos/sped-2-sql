use crate::models::traits::Model;
use efd::FILE_STRUCTURE;
use std::{collections::HashMap, pin::Pin};
use crate::SpedType;

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

pub mod efd {
    use crate::create_loader;
    use crate::models::traits::Model;
    use indexmap::IndexMap;
    use once_cell::sync::Lazy;
    use std::pin::Pin;

    use super::LoadModelFn;
    use super::RegistryEntry;

    pub static FILE_STRUCTURE: Lazy<IndexMap<&str, RegistryEntry>> = Lazy::new(|| {
        IndexMap::from([
            (
                "0000",
                RegistryEntry {
                    level: 0,
                    load_model: create_loader!(crate::models::efd::efd_0000::Efd0000),
                },
            ),
            (
                "0001",
                RegistryEntry {
                    level: 1,
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
                    load_model: create_loader!(crate::models::efd::efd_c175::EfdC175),
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
                    load_model: create_loader!(crate::models::efd::efd_c860::EfdC860),
                },
            ),
            (
                "C870",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::efd::efd_c870::EfdC870),
                },
            ),
            (
                "C880",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::efd::efd_c880::EfdC880),
                },
            ),
            (
                "C890",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::efd::efd_c890::EfdC890),
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
}

pub mod icms_ipi {
    use crate::create_loader;
    use crate::models::traits::Model;
    use indexmap::IndexMap;
    use once_cell::sync::Lazy;
    use std::pin::Pin;

    use super::LoadModelFn;
    use super::RegistryEntry;

    pub static FILE_STRUCTURE: Lazy<IndexMap<&str, RegistryEntry>> = Lazy::new(|| {
        IndexMap::from([
            (
                "0000",
                RegistryEntry {
                    level: 0,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0000::Reg0000),
                },
            ),
            (
                "0001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0001::Reg0001),
                },
            ),
            (
                "0002",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0002::Reg0002),
                },
            ),
            (
                "0005",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0005::Reg0005),
                },
            ),
            (
                "0015",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0015::Reg0015),
                },
            ),
            (
                "0100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0100::Reg0100),
                },
            ),
            (
                "0150",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0150::Reg0150),
                },
            ),
            (
                "0175",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0175::Reg0175),
                },
            ),
            (
                "0190",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0190::Reg0190),
                },
            ),
            (
                "0200",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0200::Reg0200),
                },
            ),
            (
                "0205",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0205::Reg0205),
                },
            ),
            (
                "0206",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0206::Reg0206),
                },
            ),
            (
                "0210",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0210::Reg0210),
                },
            ),
            (
                "0220",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0220::Reg0220),
                },
            ),
            (
                "0300",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0300::Reg0300),
                },
            ),
            (
                "0305",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0305::Reg0305),
                },
            ),
            (
                "0400",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0400::Reg0400),
                },
            ),
            (
                "0450",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0450::Reg0450),
                },
            ),
            (
                "0460",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0460::Reg0460),
                },
            ),
            (
                "0500",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0500::Reg0500),
                },
            ),
            (
                "0600",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0600::Reg0600),
                },
            ),
            (
                "0990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_0990::Reg0990),
                },
            ),
            (
                "B001",
                RegistryEntry {
                    level: 1,
                    load_model: None,
                },
            ),
            (
                "B990",
                RegistryEntry {
                    level: 1,
                    load_model: None,
                },
            ),
            (
                "C001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c001::RegC001),
                },
            ),
            (
                "C100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c100::RegC100),
                },
            ),
            (
                "C101",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c101::RegC101),
                },
            ),
            (
                "C105",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c105::RegC105),
                },
            ),
            (
                "C110",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c110::RegC110),
                },
            ),
            (
                "C111",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c111::RegC111),
                },
            ),
            (
                "C112",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c112::RegC112),
                },
            ),
            (
                "C113",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c113::RegC113),
                },
            ),
            (
                "C114",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c114::RegC114),
                },
            ),
            (
                "C115",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c115::RegC115),
                },
            ),
            (
                "C116",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c116::RegC116),
                },
            ),
            (
                "C120",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c120::RegC120),
                },
            ),
            (
                "C130",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c130::RegC130),
                },
            ),
            (
                "C140",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c140::RegC140),
                },
            ),
            (
                "C141",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c141::RegC141),
                },
            ),
            (
                "C160",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c160::RegC160),
                },
            ),
            (
                "C165",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c165::RegC165),
                },
            ),
            (
                "C170",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c170::RegC170),
                },
            ),
            (
                "C171",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c171::RegC171),
                },
            ),
            (
                "C172",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c172::RegC172),
                },
            ),
            (
                "C173",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c173::RegC173),
                },
            ),
            (
                "C174",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c174::RegC174),
                },
            ),
            (
                "C175",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c175::RegC175),
                },
            ),
            (
                "C176",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c176::RegC176),
                },
            ),
            (
                "C177",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c177::RegC177),
                },
            ),
            (
                "C178",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c178::RegC178),
                },
            ),
            (
                "C179",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c179::RegC179),
                },
            ),
            (
                "C190",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c190::RegC190),
                },
            ),
            (
                "C195",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c195::RegC195),
                },
            ),
            (
                "C197",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c197::RegC197),
                },
            ),
            (
                "C300",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c300::RegC300),
                },
            ),
            (
                "C310",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c310::RegC310),
                },
            ),
            (
                "C320",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c320::RegC320),
                },
            ),
            (
                "C321",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c321::RegC321),
                },
            ),
            (
                "C350",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c350::RegC350),
                },
            ),
            (
                "C370",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c370::RegC370),
                },
            ),
            (
                "C390",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c390::RegC390),
                },
            ),
            (
                "C400",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c400::RegC400),
                },
            ),
            (
                "C405",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c405::RegC405),
                },
            ),
            (
                "C410",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c410::RegC410),
                },
            ),
            (
                "C420",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c420::RegC420),
                },
            ),
            (
                "C425",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c425::RegC425),
                },
            ),
            (
                "C460",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c460::RegC460),
                },
            ),
            (
                "C465",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c465::RegC465),
                },
            ),
            (
                "C470",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c470::RegC470),
                },
            ),
            (
                "C490",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c490::RegC490),
                },
            ),
            (
                "C495",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c495::RegC495),
                },
            ),
            (
                "C500",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c500::RegC500),
                },
            ),
            (
                "C510",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c510::RegC510),
                },
            ),
            (
                "C590",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c590::RegC590),
                },
            ),
            (
                "C600",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c600::RegC600),
                },
            ),
            (
                "C601",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c601::RegC601),
                },
            ),
            (
                "C610",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c610::RegC610),
                },
            ),
            (
                "C690",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c690::RegC690),
                },
            ),
            (
                "C700",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c700::RegC700),
                },
            ),
            (
                "C790",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c790::RegC790),
                },
            ),
            (
                "C791",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c791::RegC791),
                },
            ),
            (
                "C800",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c800::RegC800),
                },
            ),
            (
                "C850",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c850::RegC850),
                },
            ),
            (
                "C860",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c860::RegC860),
                },
            ),
            (
                "C890",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c890::RegC890),
                },
            ),
            (
                "C990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_c990::RegC990),
                },
            ),
            (
                "D001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d001::RegD001),
                },
            ),
            (
                "D100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d100::RegD100),
                },
            ),
            (
                "D101",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d101::RegD101),
                },
            ),
            (
                "D110",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d110::RegD110),
                },
            ),
            (
                "D120",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d120::RegD120),
                },
            ),
            (
                "D130",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d130::RegD130),
                },
            ),
            (
                "D140",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d140::RegD140),
                },
            ),
            (
                "D150",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d150::RegD150),
                },
            ),
            (
                "D160",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d160::RegD160),
                },
            ),
            (
                "D161",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d161::RegD161),
                },
            ),
            (
                "D162",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d162::RegD162),
                },
            ),
            (
                "D170",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d170::RegD170),
                },
            ),
            (
                "D180",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d180::RegD180),
                },
            ),
            (
                "D190",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d190::RegD190),
                },
            ),
            (
                "D195",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d195::RegD195),
                },
            ),
            (
                "D197",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d197::RegD197),
                },
            ),
            (
                "D300",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d300::RegD300),
                },
            ),
            (
                "D301",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d301::RegD301),
                },
            ),
            (
                "D310",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d310::RegD310),
                },
            ),
            (
                "D350",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d350::RegD350),
                },
            ),
            (
                "D355",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d355::RegD355),
                },
            ),
            (
                "D360",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d360::RegD360),
                },
            ),
            (
                "D365",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d365::RegD365),
                },
            ),
            (
                "D370",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d370::RegD370),
                },
            ),
            (
                "D390",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d390::RegD390),
                },
            ),
            (
                "D400",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d400::RegD400),
                },
            ),
            (
                "D410",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d410::RegD410),
                },
            ),
            (
                "D411",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d411::RegD411),
                },
            ),
            (
                "D420",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d420::RegD420),
                },
            ),
            (
                "D500",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d500::RegD500),
                },
            ),
            (
                "D510",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d510::RegD510),
                },
            ),
            (
                "D530",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d530::RegD530),
                },
            ),
            (
                "D590",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d590::RegD590),
                },
            ),
            (
                "D600",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d600::RegD600),
                },
            ),
            (
                "D610",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d610::RegD610),
                },
            ),
            (
                "D690",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d690::RegD690),
                },
            ),
            (
                "D695",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d695::RegD695),
                },
            ),
            (
                "D696",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d696::RegD696),
                },
            ),
            (
                "D697",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d697::RegD697),
                },
            ),
            (
                "D990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_d990::RegD990),
                },
            ),
            (
                "E001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e001::RegE001),
                },
            ),
            (
                "E100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e100::RegE100),
                },
            ),
            (
                "E110",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e110::RegE110),
                },
            ),
            (
                "E111",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e111::RegE111),
                },
            ),
            (
                "E112",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e112::RegE112),
                },
            ),
            (
                "E113",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e113::RegE113),
                },
            ),
            (
                "E115",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e115::RegE115),
                },
            ),
            (
                "E116",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e116::RegE116),
                },
            ),
            (
                "E200",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e200::RegE200),
                },
            ),
            (
                "E210",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e210::RegE210),
                },
            ),
            (
                "E220",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e220::RegE220),
                },
            ),
            (
                "E230",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e230::RegE230),
                },
            ),
            (
                "E240",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e240::RegE240),
                },
            ),
            (
                "E250",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e250::RegE250),
                },
            ),
            (
                "E500",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e500::RegE500),
                },
            ),
            (
                "E510",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e510::RegE510),
                },
            ),
            (
                "E520",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e520::RegE520),
                },
            ),
            (
                "E530",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e530::RegE530),
                },
            ),
            (
                "E990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_e990::RegE990),
                },
            ),
            (
                "G001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g001::RegG001),
                },
            ),
            (
                "G110",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g110::RegG110),
                },
            ),
            (
                "G125",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g125::RegG125),
                },
            ),
            (
                "G126",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g126::RegG126),
                },
            ),
            (
                "G130",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g130::RegG130),
                },
            ),
            (
                "G140",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g140::RegG140),
                },
            ),
            (
                "G990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_g990::RegG990),
                },
            ),
            (
                "H001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_h001::RegH001),
                },
            ),
            (
                "H005",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_h005::RegH005),
                },
            ),
            (
                "H010",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_h010::RegH010),
                },
            ),
            (
                "H020",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_h020::RegH020),
                },
            ),
            (
                "H990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_h990::RegH990),
                },
            ),
            (
                "K001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k001::RegK001),
                },
            ),
            (
                "K100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k100::RegK100),
                },
            ),
            (
                "K200",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k200::RegK200),
                },
            ),
            (
                "K220",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k220::RegK220),
                },
            ),
            (
                "K230",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k230::RegK230),
                },
            ),
            (
                "K235",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k235::RegK235),
                },
            ),
            (
                "K250",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k250::RegK250),
                },
            ),
            (
                "K255",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k255::RegK255),
                },
            ),
            (
                "K990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_k990::RegK990),
                },
            ),
            (
                "1001",
                RegistryEntry {
                    level: 1,
                    load_model: None,
                },
            ),
            (
                "1010",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1010::Reg1010),
                },
            ),
            (
                "1100",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1100::Reg1100),
                },
            ),
            (
                "1105",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1105::Reg1105),
                },
            ),
            (
                "1110",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1110::Reg1110),
                },
            ),
            (
                "1200",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1200::Reg1200),
                },
            ),
            (
                "1210",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1210::Reg1210),
                },
            ),
            (
                "1300",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1300::Reg1300),
                },
            ),
            (
                "1310",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1310::Reg1310),
                },
            ),
            (
                "1320",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1320::Reg1320),
                },
            ),
            (
                "1350",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1350::Reg1350),
                },
            ),
            (
                "1360",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1360::Reg1360),
                },
            ),
            (
                "1370",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1370::Reg1370),
                },
            ),
            (
                "1390",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1390::Reg1390),
                },
            ),
            (
                "1391",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1391::Reg1391),
                },
            ),
            (
                "1400",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1400::Reg1400),
                },
            ),
            (
                "1500",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1500::Reg1500),
                },
            ),
            (
                "1510",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1510::Reg1510),
                },
            ),
            (
                "1600",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1600::Reg1600),
                },
            ),
            (
                "1700",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1700::Reg1700),
                },
            ),
            (
                "1710",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1710::Reg1710),
                },
            ),
            (
                "1800",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1800::Reg1800),
                },
            ),
            (
                "1900",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1900::Reg1900),
                },
            ),
            (
                "1910",
                RegistryEntry {
                    level: 3,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1910::Reg1910),
                },
            ),
            (
                "1920",
                RegistryEntry {
                    level: 4,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1920::Reg1920),
                },
            ),
            (
                "1921",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1921::Reg1921),
                },
            ),
            (
                "1922",
                RegistryEntry {
                    level: 6,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1922::Reg1922),
                },
            ),
            (
                "1923",
                RegistryEntry {
                    level: 6,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1923::Reg1923),
                },
            ),
            (
                "1925",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1925::Reg1925),
                },
            ),
            (
                "1926",
                RegistryEntry {
                    level: 5,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1926::Reg1926),
                },
            ),
            (
                "1990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_1990::Reg1990),
                },
            ),
            (
                "9001",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_9001::Reg9001),
                },
            ),
            (
                "9900",
                RegistryEntry {
                    level: 2,
                    load_model: create_loader!(crate::models::icms_ipi::reg_9900::Reg9900),
                },
            ),
            (
                "9990",
                RegistryEntry {
                    level: 1,
                    load_model: create_loader!(crate::models::icms_ipi::reg_9990::Reg9990),
                },
            ),
            (
                "9999",
                RegistryEntry {
                    level: 0,
                    load_model: create_loader!(crate::models::icms_ipi::reg_9999::Reg9999),
                },
            ),
        ])
    });
}

pub fn get_reg_children(sped_type: SpedType) -> HashMap<String, Vec<String>> {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut parent_stack: Vec<(String, u8)> = Vec::new(); // Stack of (reg_code, level)

    let data = if matches!(sped_type, SpedType::Efd) {
        FILE_STRUCTURE.iter()
    } else {
        icms_ipi::FILE_STRUCTURE.iter()
    };

    for (reg, structure) in data {
        while let Some(&(_, parent_level)) = parent_stack.last() {
            if parent_level >= structure.level {
                parent_stack.pop();
            } else {
                break;
            }
        }

        if let Some((parent_reg, _)) = parent_stack.last() {
            if structure.level == parent_stack.last().unwrap().1 + 1 {
                children
                    .entry(parent_reg.to_string())
                    .or_default()
                    .push(reg.to_string());
            }
        }

        parent_stack.push((reg.to_string(), structure.level));
    }

    children
}
