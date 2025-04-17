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

#[tokio::main]
async fn main() {
    let db = database::setup_database(false).await;

    // let files = vec!["/Users/jarbassantos/Downloads/12.2021-SPEDCONTRIBUICEES.txt"];
    let mut files = fs::read_dir("/home/jarbassantos/Downloads/Caio/ORIGINAIS")
        .await
        .unwrap();

    let mut file_index = 0;
    let mut tasks = vec![];

    while let Ok(Some(entry)) = files.next_entry().await {
        file_index += 1;

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
                    .await
                    .expect("Failed to read line");

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
                let last_insert_id = if let Some(Ok(result)) = inserted_line {
                    result.last_insert_rowid()
                } else {
                    parent_id.unwrap_or(0)
                };

                let reg_code = line[1..5].to_string();
                let hierarchy = utils::reg_hierarchy::HIERARCHY.get(&reg_code.as_str());
                let level = match hierarchy.is_some() {
                    true => hierarchy.unwrap().level,
                    false => 1,
                };

                if level > parent_level.unwrap_or(0) {
                    parent_id = Some(last_insert_id);
                    parent_level = Some(level);
                }
            }

            println!("File {} finished", file_index)
        });

        tasks.push(task);
    }

    let _ = join_all(tasks).await;
}
