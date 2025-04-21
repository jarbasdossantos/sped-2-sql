mod database;
mod models;
mod sped;
mod utils;

use encoding_rs::UTF_8;
use sped::handle_line;
use tokio::fs::{self, File};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task;
use futures::future::join_all;
use anyhow::Result;
use tokio::sync::mpsc;

struct ProgressMessage {
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<ProgressMessage>(100);

    let processing = task::spawn(async move {
        process_files(vec![
            "/Users/jarbassantos/Downloads/Caio/Espelhador/est/30408862000119-119421304116-20190901-20190930-0-738705006D7A6026DD42B54846911A6F7CC6E455-SPED-EFD.txt".to_string(),
            // "/Users/jarbassantos/Downloads/Caio/Espelhador/FED/PISCOFINS_20240601_20240630_30408862000119_Original_20240723083807_E49E4835A419E8C70DC8EC9130C8301FCDEFCEB4.txt".to_string(),
            // "/Users/jarbassantos/Downloads/Caio/Espelhador/FED/PISCOFINS_20240501_20240531_30408862000119_Original_20240625110827_3726424191B5281A533E0F4B478A33F3207AFDF1.txt".to_string(),
        ], tx).await
    });

    while let Some(msg) = rx.recv().await {
        println!("{}", msg.message);
    }

    processing.await??;

    Ok(())
}

async fn process_files(files: Vec<String>, tx: mpsc::Sender<ProgressMessage>) -> Result<(), anyhow::Error> {
    let db = database::setup_database(false).await;
    let mut tasks = vec![];

    for file in files {
        let extension = file
            .split('.')
            .last()
            .map(|s| s.to_string())
            .unwrap();

        let file_name = file
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap_or("")
            .to_string();


        if extension != "txt" {
            continue;
        }

        let tx_clone = tx.clone();
        let db_clone = db.clone();
        let file_clone = file.clone();

        let task = task::spawn(async move {
            tx_clone.send(ProgressMessage { message: format!("Processing file: {}", file_name) }).await.unwrap();

            let file = match File::open(file_clone).await {
                Ok(file) => file,
                Err(err) => {
                    let message = format!("Failed to open file: {}", err);
                    tx_clone.send(ProgressMessage { message }).await.unwrap();
                    return;
                }
            };

            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            let mut parent_id = 0i64;
            let mut parent_level = 0u8;

            loop {
                buffer.clear();

                let bytes_read = reader
                    .read_until(b'\n', &mut buffer)
                    .await.unwrap();

                if bytes_read == 0 {
                    break;
                }

                if let Some(&b'\n') = buffer.last() {
                    buffer.pop();
                }

                if let Some(&b'\r') = buffer.last() {
                    buffer.pop();
                }

                let (line, _, _) = UTF_8.decode(&buffer);

                if !line.ends_with("|") {
                    continue;
                }

                let reg_code = line[1..5].to_string();
                let hierarchy = utils::reg_hierarchy::HIERARCHY.get(&reg_code.as_str());
                let level = match hierarchy.is_some() {
                    true => hierarchy.unwrap().level,
                    false => 1,
                };

                if level > parent_level {
                    parent_level = level;
                } else if level < parent_level {
                    parent_id = 0;
                }

                let inserted_line = handle_line(&line, parent_id, &db_clone).await;

                match inserted_line {
                    Ok(Some(result)) => {
                        parent_id = result.last_insert_rowid();
                    }
                    Err(error) => {
                        let message = format!("Failed to insert line: {}", error);
                        tx_clone.send(ProgressMessage { message }).await.unwrap();
                    }
                    _ => {}
                };
            }

            tx_clone.send(ProgressMessage { message: format!("Finished file: {}", file_name) }).await.unwrap();
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    tx.send(ProgressMessage { message: "Finished processing files".to_string() }).await?;

    Ok(())
}
