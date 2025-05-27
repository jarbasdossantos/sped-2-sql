use crate::models::traits::{Model, ModelFactory};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref REGISTRY: Mutex<HashMap<&'static str, Box<dyn ModelFactory + Send + Sync>>> =
        Mutex::new(HashMap::new());
}

pub fn register(factory: Box<dyn ModelFactory + Send + Sync>) {
    let reg = factory.handle_reg();
    let mut registry = REGISTRY.lock().unwrap();
    registry.insert(reg, factory);
}

pub fn create_model(
    reg: &str,
    fields: Vec<&str>,
    id: Option<i32>,
    parent_id: Option<i32>,
    file_id: i32,
) -> Option<Box<dyn Model>> {
    let registry = REGISTRY.lock().unwrap();
    registry
        .get(reg)
        .map(|f| f.create_model(fields, id, parent_id, file_id))
}

pub fn register_models() {
    crate::models::efd::efd_0000::register();
    crate::models::efd::efd_0001::register();
    crate::models::efd::efd_0035::register();
    crate::models::efd::efd_0100::register();
    crate::models::efd::efd_0110::register();
    crate::models::efd::efd_0111::register();
    crate::models::efd::efd_0120::register();
    crate::models::efd::efd_0140::register();
    crate::models::efd::efd_0145::register();
    crate::models::efd::efd_0150::register();
    crate::models::efd::efd_0190::register();
    crate::models::efd::efd_0200::register();
    crate::models::efd::efd_0205::register();
    crate::models::efd::efd_0206::register();
    crate::models::efd::efd_0208::register();
    crate::models::efd::efd_0400::register();
    crate::models::efd::efd_0450::register();
    crate::models::efd::efd_0500::register();
    crate::models::efd::efd_0600::register();
    crate::models::efd::efd_0990::register();
    crate::models::efd::efd_1001::register();
    crate::models::efd::efd_1010::register();
    crate::models::efd::efd_1020::register();
    crate::models::efd::efd_1100::register();
    crate::models::efd::efd_1300::register();
    crate::models::efd::efd_1500::register();
    crate::models::efd::efd_1700::register();
    crate::models::efd::efd_1800::register();
    crate::models::efd::efd_1900::register();
    crate::models::efd::efd_1990::register();
    crate::models::efd::efd_9001::register();
    crate::models::efd::efd_9900::register();
    crate::models::efd::efd_9990::register();
    crate::models::efd::efd_9999::register();
    crate::models::efd::efd_a001::register();
    crate::models::efd::efd_a010::register();
    crate::models::efd::efd_a100::register();
    crate::models::efd::efd_a110::register();
    crate::models::efd::efd_a111::register();
    crate::models::efd::efd_a120::register();
    crate::models::efd::efd_a170::register();
    crate::models::efd::efd_a990::register();
    crate::models::efd::efd_c001::register();
    crate::models::efd::efd_c010::register();
    crate::models::efd::efd_c100::register();
    crate::models::efd::efd_c110::register();
    crate::models::efd::efd_c111::register();
    crate::models::efd::efd_c120::register();
    crate::models::efd::efd_c170::register();
    crate::models::efd::efd_c180::register();
    crate::models::efd::efd_c181::register();
    crate::models::efd::efd_c185::register();
    crate::models::efd::efd_c188::register();
    crate::models::efd::efd_c190::register();
    crate::models::efd::efd_c191::register();
    crate::models::efd::efd_c195::register();
    crate::models::efd::efd_c198::register();
    crate::models::efd::efd_c199::register();
    crate::models::efd::efd_c380::register();
    crate::models::efd::efd_c381::register();
    crate::models::efd::efd_c385::register();
    crate::models::efd::efd_c395::register();
    crate::models::efd::efd_c396::register();
    crate::models::efd::efd_c400::register();
    crate::models::efd::efd_c405::register();
    crate::models::efd::efd_c481::register();
    crate::models::efd::efd_c485::register();
    crate::models::efd::efd_c489::register();
    crate::models::efd::efd_c490::register();
    crate::models::efd::efd_c491::register();
    crate::models::efd::efd_c495::register();
    crate::models::efd::efd_c499::register();
    crate::models::efd::efd_c500::register();
    crate::models::efd::efd_c501::register();
    crate::models::efd::efd_c505::register();
    crate::models::efd::efd_c509::register();
    crate::models::efd::efd_c600::register();
    crate::models::efd::efd_c601::register();
    crate::models::efd::efd_c605::register();
    crate::models::efd::efd_c609::register();
    crate::models::efd::efd_c990::register();
    crate::models::efd::efd_d001::register();
    crate::models::efd::efd_d010::register();
    crate::models::efd::efd_d100::register();
    crate::models::efd::efd_d101::register();
    crate::models::efd::efd_d105::register();
    crate::models::efd::efd_d111::register();
    crate::models::efd::efd_d200::register();
    crate::models::efd::efd_d201::register();
    crate::models::efd::efd_d205::register();
    crate::models::efd::efd_d209::register();
    crate::models::efd::efd_d300::register();
    crate::models::efd::efd_d309::register();
    crate::models::efd::efd_d350::register();
    crate::models::efd::efd_d359::register();
    crate::models::efd::efd_d500::register();
    crate::models::efd::efd_d501::register();
    crate::models::efd::efd_d505::register();
    crate::models::efd::efd_d509::register();
    crate::models::efd::efd_d600::register();
    crate::models::efd::efd_d601::register();
    crate::models::efd::efd_d605::register();
    crate::models::efd::efd_d609::register();
    crate::models::efd::efd_d990::register();
    crate::models::efd::efd_f001::register();
    crate::models::efd::efd_f010::register();
    crate::models::efd::efd_f100::register();
    crate::models::efd::efd_f111::register();
    crate::models::efd::efd_f120::register();
    crate::models::efd::efd_f129::register();
    crate::models::efd::efd_f130::register();
    crate::models::efd::efd_f139::register();
    crate::models::efd::efd_f150::register();
    crate::models::efd::efd_f200::register();
    crate::models::efd::efd_f205::register();
    crate::models::efd::efd_f210::register();
    crate::models::efd::efd_f211::register();
    crate::models::efd::efd_f500::register();
    crate::models::efd::efd_f509::register();
    crate::models::efd::efd_f510::register();
    crate::models::efd::efd_f519::register();
    crate::models::efd::efd_f525::register();
    crate::models::efd::efd_f550::register();
    crate::models::efd::efd_f559::register();
    crate::models::efd::efd_f560::register();
    crate::models::efd::efd_f569::register();
    crate::models::efd::efd_f600::register();
    crate::models::efd::efd_f700::register();
    crate::models::efd::efd_f800::register();
    crate::models::efd::efd_f990::register();
    crate::models::efd::efd_i001::register();
    crate::models::efd::efd_i010::register();
    crate::models::efd::efd_i100::register();
    crate::models::efd::efd_i199::register();
    crate::models::efd::efd_i200::register();
    crate::models::efd::efd_i299::register();
    crate::models::efd::efd_i300::register();
    crate::models::efd::efd_i399::register();
    crate::models::efd::efd_i990::register();
    crate::models::efd::efd_m001::register();
    crate::models::efd::efd_m100::register();
    crate::models::efd::efd_m105::register();
    crate::models::efd::efd_m110::register();
    crate::models::efd::efd_m200::register();
    crate::models::efd::efd_m205::register();
    crate::models::efd::efd_m210::register();
    crate::models::efd::efd_m211::register();
    crate::models::efd::efd_m220::register();
    crate::models::efd::efd_m230::register();
    crate::models::efd::efd_m300::register();
    crate::models::efd::efd_m350::register();
    crate::models::efd::efd_m400::register();
    crate::models::efd::efd_m410::register();
    crate::models::efd::efd_m500::register();
    crate::models::efd::efd_m505::register();
    crate::models::efd::efd_m510::register();
    crate::models::efd::efd_m600::register();
    crate::models::efd::efd_m605::register();
    crate::models::efd::efd_m610::register();
    crate::models::efd::efd_m611::register();
    crate::models::efd::efd_m620::register();
    crate::models::efd::efd_m630::register();
    crate::models::efd::efd_m700::register();
    crate::models::efd::efd_m800::register();
    crate::models::efd::efd_m810::register();
    crate::models::efd::efd_m990::register();
    crate::models::efd::efd_p001::register();
    crate::models::efd::efd_p010::register();
    crate::models::efd::efd_p100::register();
    crate::models::efd::efd_p110::register();
    crate::models::efd::efd_p199::register();
    crate::models::efd::efd_p200::register();
    crate::models::efd::efd_p210::register();
    crate::models::efd::efd_p990::register();
}
