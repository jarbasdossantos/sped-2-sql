use sped_to_database::{ExportFile, ImportFilesData, SpedType};

#[tokio::main]
async fn main() {
    sped_to_database::clean().await;

    sped_to_database::import_files(sped_to_database::ImportFiles {
        files: vec![
            ImportFilesData {
                file: "/Users/jarbassantos/Customers/Projects/caio-1/PISCOFINS_20210901_20210930_13773225000122_Original_20211116163114_44C0AFE5344FF5317996A62EF3FEB05A0A5B9F69.txt".to_string(),
                registers: None,
                sped_type: SpedType::IcmsIpi,
            }
        ],
        progress_tx: None,
    }).await;

    println!("\nImported");

    let files = sped_to_database::export(ExportFile {
        id: 1,
        registers: None,
        sped_type: SpedType::IcmsIpi,
    })
    .await;

    for file in files {
        for model in file {
            // print!("{}", model);
        }
    }
}
