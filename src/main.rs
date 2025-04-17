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

#[tokio::main]
async fn main() -> Result<()> {
    let db = database::setup_database(false).await;

    // let files = vec!["/Users/jarbassantos/Downloads/12.2021-SPEDCONTRIBUICEES.txt"];
    let mut files = fs::read_dir("/Users/jarbassantos/Downloads/Caio/2021")
        .await
        .unwrap();

    let mut tasks = vec![];

    while let Some(entry) = files.next_entry().await? {
        let file_path = entry.path();

        if file_path.extension() != Some("txt".as_ref()) {
            continue;
        }

        let db = db.clone();

        let task = task::spawn(async move {
            let file = File::open(file_path).await.expect("Failed to open file");
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            let mut parent_id: Option<i64> = None;
            let mut parent_level: Option<u8> = None;

            loop {
                buffer.clear();

                let bytes_read = reader
                    .read_until(b'\n', &mut buffer)
                    .await?;

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

                let inserted_line = handle_line(&line, parent_id, &db).await;
                let insert_id = match inserted_line {
                    Ok(Some(result)) => Some(result.last_insert_rowid()),
                    Ok(None) => None,
                    Err(error) => return Err(error)
                };

                let reg_code = line[1..5].to_string();
                let hierarchy = utils::reg_hierarchy::HIERARCHY.get(&reg_code.as_str());
                let level = match hierarchy.is_some() {
                    true => hierarchy.unwrap().level,
                    false => 1,
                };

                if insert_id.is_some() && level > parent_level.unwrap_or(0) {
                    parent_id = insert_id;
                    parent_level = Some(level);
                }
            }

            Ok(())
        });

        tasks.push(task);
    }

    let mut errors = vec![];
    for result in join_all(tasks).await {
        match result {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                errors.push(format!("Task error: {}", e));
            }
            Err(join_err) => {
                errors.push(format!("Task panicked: {}", join_err));
            }
        }
    }

    if !errors.is_empty() {
        for err in &errors {
            eprintln!("{}", err);
        }
    }

    Ok(())
}
