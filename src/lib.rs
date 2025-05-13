mod database;
mod models;
mod sped;
mod utils;

use crate::models::files::Files;
use crate::models::files::FilesTrait;
use anyhow::Result;
use database::DB_POOL;
use encoding_rs::UTF_8;
use futures::future::join_all;
use models::traits::Reg;
use sped::handle_line;
use std::time::SystemTime;
use time::{format_description, OffsetDateTime};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::task;

struct ProgressMessage {
    message: String,
}

pub struct Load {
    pub files: Vec<String>,
}

pub struct Export {
    pub id: i64,
}

pub async fn load(data: Load) -> Result<(), anyhow::Error> {
    let (tx, mut rx) = mpsc::channel::<ProgressMessage>(100);
    let processing = task::spawn(async move { import_files(data.files, tx).await });

    while let Some(msg) = rx.recv().await {
        println!("{}", msg.message);
    }

    processing.await??;

    Ok(())
}

pub async fn export(data: Export) -> Result<Vec<Box<dyn Reg>>, anyhow::Error> {
    let file_data = Files::get_data(data.id).await?;

    return Ok(file_data);
}

async fn import_files(
    files: Vec<String>,
    tx: mpsc::Sender<ProgressMessage>,
) -> Result<(), anyhow::Error> {
    database::migrate().await;

    let mut tasks = vec![];

    if files.is_empty() {
        tx.send(ProgressMessage {
            message: "No files to process".to_string(),
        })
        .await?;

        return Ok(());
    }

    for file in files {
        let extension = file.split('.').last().map(|s| s.to_string()).unwrap();

        let file_name = file
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap_or("")
            .to_string();

        let file_id = match create_file_entry(file_name.clone()).await {
            Ok(result) => result.last_insert_rowid(),
            Err(err) => {
                let message = format!("Failed to create file entry: {}", err);
                tx.send(ProgressMessage { message }).await?;
                continue;
            }
        };

        if extension != "txt" {
            continue;
        }

        let tx_clone = tx.clone();
        let file_clone = file.clone();

        // Run each file in an async separate task
        let task = task::spawn(async move {
            if let Err(err) = import(file_clone, file_name, file_id, tx_clone).await {
                let message = format!("Failed to process file: {}", err);
                eprintln!("{message}");
            }
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

async fn import(
    file_path: String,
    file_name: String,
    file_id: i64,
    tx: mpsc::Sender<ProgressMessage>,
) -> Result<(), anyhow::Error> {
    tx.send(ProgressMessage {
        message: format!("{}: Processing file: {}", get_time(), file_name),
    })
    .await?;

    let file = File::open(file_path).await?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Stack of (level, parent_id)
    let mut parent_stack: Vec<(u8, i64)> = Vec::new();

    loop {
        buffer.clear();

        let bytes_read = reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Failed to read from file");

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
        let hierarchy = utils::file_structure::FILE_STRUCTURE.get(&reg_code.as_str());
        let level = match hierarchy.is_some() {
            true => hierarchy.unwrap().level,
            false => 1,
        };

        // Get the correct parent_id using the stack
        while let Some(&(stack_level, _)) = parent_stack.last() {
            if stack_level >= level {
                parent_stack.pop();
            } else {
                break;
            }
        }
        let parent_id = parent_stack.last().map(|&(_, id)| id).unwrap_or(0);
        let inserted_line = handle_line(&line, parent_id, file_id).await;

        match inserted_line {
            Ok(Some(result)) => {
                // Add inserted id in the stack to be used for a potential child
                parent_stack.push((level, result.last_insert_rowid()));
            }
            Err(error) => {
                let message = format!("Failed to insert line ({}): {}", reg_code, error);
                tx.send(ProgressMessage { message }).await?;
            }
            _ => {}
        };
    }

    tx.send(ProgressMessage {
        message: format!("{}: Finished file: {}", get_time(), file_name),
    })
    .await?;

    Ok(())
}

async fn create_file_entry(
    file_name: String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO files (name) VALUES (?)")
        .bind(&file_name)
        .execute(&*DB_POOL)
        .await
}

fn get_time() -> String {
    let time: OffsetDateTime = SystemTime::now().into();
    let format: &'static str = "[hour]:[minute]:[second]";
    let time_format = format_description::parse(format).unwrap();

    time.format(&time_format).unwrap().to_string()
}
