use diesel::{sql_query, Queryable, QueryableByName, RunQueryDsl};
use sped_to_database::{ExportFile, ImportFilesData, SpedType};
use sped_to_database::models::icms_ipi;
use sped_to_database::models::icms_ipi::reg_c100::RegC100;
use sped_to_database::database::DB_POOL;
use diesel::sql_types::{Nullable, Text};

#[derive(Debug, Queryable, QueryableByName)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct Teste {
    #[sql_type = "Nullable<Text>"]
    invoices_ids: Option<String>,
}

#[tokio::main]
async fn main() {
    let results = sql_query(
        "SELECT group_concat(reg_c100.id || '') AS invoices_ids FROM reg_c100
                INNER JOIN efd_c100 ON efd_c100.chv_nfe = reg_c100.chv_nfe")
        .get_result::<Teste>(&mut DB_POOL.get().unwrap());

    println!("{:#?}", results);

    // sped_to_database::clean().await;
    //
    // sped_to_database::import_files(sped_to_database::ImportFiles {
    //     files: vec![
    //         ImportFilesData {
    //             file: "/Users/jarbassantos/Downloads/PISCOFINS_20200601_20200630_12662352000191_Original_20200812091255_D08544F39B95698618D6E66F9FACB1BDB7768BD0.txt".to_string(),
    //             registers: None,
    //             sped_type: SpedType::Efd,
    //         }
    //     ],
    //     progress_tx: None,
    // }).await;
    //
    // println!("\nImported");
    //
    // let files = sped_to_database::export(ExportFile {
    //     id: 1,
    //     registers: None,
    //     sped_type: SpedType::IcmsIpi,
    // })
    // .await;
    //
    // for file in files {
    //     for model in file {
    //         // print!("{}", model);
    //     }
    // }
}
