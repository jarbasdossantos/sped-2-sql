mod database;
mod models;
mod sped;
mod utils;

use anyhow::Result;
use encoding_rs::UTF_8;
use futures::future::join_all;
use sped::handle_line;
use sqlx::{Pool, Sqlite};
use std::fmt::Debug;
use std::time::SystemTime;
use time::{format_description, OffsetDateTime};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::task;

struct ProgressMessage {
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<ProgressMessage>(100);

    let processing = task::spawn(async move {
        process_files(
            vec![
                "/home/jarbassantos/Downloads/Caio/ORIGINAIS/19(12).txt".to_string(),
                "/home/jarbassantos/Downloads/Caio/ORIGINAIS/20(1).txt".to_string(),
                "/home/jarbassantos/Downloads/Caio/ORIGINAIS/20(2).txt".to_string(),
            ],
            tx,
        )
        .await
    });

    while let Some(msg) = rx.recv().await {
        println!("{}", msg.message);
    }

    processing.await??;

    Ok(())
}

async fn process_files(
    files: Vec<String>,
    tx: mpsc::Sender<ProgressMessage>,
) -> Result<(), anyhow::Error> {
    let db = database::setup_database(false).await;
    let mut tasks = vec![];

    if files.is_empty() {
        tx.send(ProgressMessage {
            message: "No files to process".to_string(),
        })
        .await
        .unwrap();
        return Ok(());
    }

    for file in files {
        let extension = file.split('.').last().map(|s| s.to_string()).unwrap();

        let file_name = file
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap_or("")
            .to_string();

        let file_id = match create_file_entry(db.clone(), file_name.clone()).await {
            Ok(result) => result.last_insert_rowid(),
            Err(err) => {
                let message = format!("Failed to create file entry: {}", err);
                tx.send(ProgressMessage { message }).await.unwrap();
                continue;
            }
        };

        if extension != "txt" {
            continue;
        }

        let tx_clone = tx.clone();
        let db_clone = db.clone();
        let file_clone = file.clone();

        let task = task::spawn(async move {
            let time: OffsetDateTime = SystemTime::now().into();

            tx_clone
                .send(ProgressMessage {
                    message: format!("{}: Processing file: {}", get_time(), file_name),
                })
                .await
                .unwrap();

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

                let bytes_read = reader.read_until(b'\n', &mut buffer).await.unwrap();

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

                if level < parent_level {
                    parent_id = 0;
                }

                let inserted_line = handle_line(&line, parent_id, file_id, &db_clone).await;

                match inserted_line {
                    Ok(Some(result)) => {
                        parent_id = result.last_insert_rowid();
                    }
                    Err(error) => {
                        let message = format!("Failed to insert line ({}): {}", reg_code, error);
                        tx_clone.send(ProgressMessage { message }).await.unwrap();
                    }
                    _ => {}
                };
            }

            tx_clone
                .send(ProgressMessage {
                    message: format!("{}: Finished file: {}", get_time(), file_name),
                })
                .await
                .unwrap();
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    tx.send(ProgressMessage {
        message: "Finished processing files".to_string(),
    })
    .await?;

    Ok(())
}

async fn create_file_entry(
    db: Pool<Sqlite>,
    file_name: String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO files (name) VALUES (?)")
        .bind(&file_name)
        .execute(&db)
        .await
}

async fn get_parent_id(
    db: Pool<Sqlite>,
    reg_code: String,
    parent_id: &mut i64,
    parent_level: &mut u8,
) -> Result<i64, sqlx::Error> {
    let hierarchy = utils::reg_hierarchy::HIERARCHY.get(&reg_code.as_str());

    let level = match hierarchy.is_some() {
        true => hierarchy.unwrap().level,
        false => 1,
    };

    if level >= *parent_level {
        return Ok(*parent_id);
    }

    if level < *parent_level {
        sqlx::query("SELECT id FROM reg WHERE reg = ?")
            .bind(reg_code)
            .fetch_one(&db)
            .await
            .map(|row| {
                *parent_id = row.get(0);
                *parent_level = level;
                row.get(0)
            })
    }

    Ok(0)
}

fn get_time() -> String {
    let time: OffsetDateTime = SystemTime::now().into();
    let format: &'static str = "[hour]:[minute]:[second]";
    let time_format = format_description::parse(format).unwrap();

    time.format(&time_format).unwrap().to_string()
}
