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
use log::error;
use models::traits::Reg;
use sped::handle_line;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task;

pub struct LoadData {
    pub file: String,
    pub registers: Option<Vec<String>>,
}

pub struct Load {
    pub files: Vec<LoadData>,
}

pub struct Export {
    pub id: i64,
    pub registers: Option<Vec<String>>,
}

pub async fn load(data: Load) -> Result<(), anyhow::Error> {
    let processing = task::spawn(async move { import_files(data.files).await });

    processing.await??;

    Ok(())
}

pub async fn export(data: Export) -> Result<Vec<Box<dyn Reg>>, anyhow::Error> {
    let file_data = Files::get_data(data).await?;

    return Ok(file_data);
}

async fn import_files(files: Vec<LoadData>) -> Result<(), anyhow::Error> {
    match database::clean().await {
        Ok(_) => {}
        Err(err) => {
            error!("Failed to clean database: {}", err);
            return Err(err);
        }
    };

    database::migrate().await;

    let mut tasks = vec![];

    if files.is_empty() {
        return Ok(());
    }

    for data in files {
        let extension = data.file.split('.').last().map(|s| s.to_string()).unwrap();

        let file_name = data
            .file
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap_or("")
            .to_string();

        let file_id = match create_file_entry(file_name.clone()).await {
            Ok(result) => result.last_insert_rowid(),
            Err(err) => {
                println!("Failed to create file entry: {}", err);
                continue;
            }
        };

        if extension != "txt" {
            continue;
        }

        let file_clone = data.file.clone();

        // Run each file in an async separate task
        let task = task::spawn(async move {
            if let Err(err) = import(file_clone, data.registers, file_id).await {
                let message = format!("Failed to process file: {}", err);
                eprintln!("{message}");
            }
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}

async fn import(
    file_path: String,
    registers: Option<Vec<String>>,
    file_id: i64,
) -> Result<(), anyhow::Error> {
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

        if let Some(ref regs) = registers {
            if !regs.contains(&reg_code) {
                continue;
            }
        }

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
                println!("Failed to insert line ({}): {}", reg_code, error);
            }
            _ => {}
        };
    }

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