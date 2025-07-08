use sped_to_database::{export, ExportFile, SpedType};
use std::env;

#[tokio::main]
pub async fn main() {
    unsafe {
        env::set_var("USE_MEMORY_DB", "false");
        env::set_var("DATABASE_URL", "/Users/jarbassantos/Library/Application Support/com.jarbas.dev.Caio/database.sqlite");
    }

    let file_data = export(ExportFile {
        file_id: 1,
        registers: None,
        sped_type: SpedType::Efd,
    }).await.unwrap();

    for model in file_data {
        println!("{model}");
    }
}