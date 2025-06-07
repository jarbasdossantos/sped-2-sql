mod database;
pub mod macros;
pub mod models;
mod schemas;
mod sped;
mod utils;

use crate::models::files::File;
use crate::models::registry::{register_efd_models, register_icms_ipi_models};
use crate::models::traits::FilesModel;
use crate::schemas::files::dsl as schema;
use crate::schemas::files::table;
use anyhow::{Error, Result};
use database::DB_POOL;
use diesel::dsl::sql;
use diesel::sql_types::Integer;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use encoding_rs::UTF_8;
use futures::future::join_all;
use log::{error, info, warn};
use models::traits::Model;
use sped::handle_line;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio::task;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SpedType {
    Efd,
    IcmsIpi,
}

pub struct ImportFilesData {
    /// File path
    pub file: String,
    /// Registers to import
    pub registers: Option<Vec<String>>,
    pub sped_type: SpedType,
}

pub struct ImportFiles {
    pub files: Vec<ImportFilesData>,
    pub progress_tx: Option<Sender<Progress>>,
}

#[derive(Debug, Clone)]
pub struct ExportFile {
    /// File id
    pub id: i32,
    /// Registers to export
    pub registers: Option<Vec<String>>,
    /// Sped type
    pub sped_type: SpedType,
}

#[derive(Debug, Clone)]
pub enum Status {
    Processing,
    Skipped { reason: String },
    ProcessingFailed { error: String },
    ProcessingSuccessful,
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub file_path: String,
    pub status: Status,
}

pub async fn export(data: ExportFile) -> Result<Vec<Box<dyn Model>>, Error> {
    if matches!(data.sped_type, SpedType::Efd) {
        register_efd_models();
    } else {
        register_icms_ipi_models();
    };

    let file_data = File::get_file_data(data).await?;

    Ok(file_data)
}

pub async fn import_files(mut data: ImportFiles) -> Result<(), Error> {
    let mut tasks = vec![];
    let semaphore = Arc::new(tokio::sync::Semaphore::new(1));

    for file_data in data.files {
        let file_path = std::path::Path::new(&file_data.file);

        send_status(
            &mut data.progress_tx,
            file_data.file.clone(),
            Status::Processing,
        )
        .await;

        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        let file_id = match create_file_entry(file_name.clone(), file_data.sped_type).await {
            Ok(file_id) => file_id,
            Err(err) => {
                let error_message = format!("Failed to create file entry: {err}");

                send_status(
                    &mut data.progress_tx,
                    file_data.file.clone(),
                    Status::Skipped {
                        reason: error_message.to_string(),
                    },
                )
                .await;

                error!("{error_message}");
                continue;
            }
        };

        if extension != "txt" {
            let error_message = "Invalid file extension. Only .txt is allowed";

            send_status(
                &mut data.progress_tx,
                file_data.file.clone(),
                Status::Skipped {
                    reason: error_message.to_string(),
                },
            )
            .await;

            continue;
        }

        let file_clone = file_data.file.clone();
        let semaphore_clone = semaphore.clone();
        let task_file_path = file_clone.clone();
        let mut task_progress_tx = data.progress_tx.clone();

        let task = task::spawn(async move {
            let _permit = semaphore_clone.acquire().await;

            info!("Importing file: {file_clone}");

            if let Err(err) = do_import(
                file_clone,
                file_data.registers,
                file_id,
                file_data.sped_type,
            )
            .await
            {
                let error_message = format!("Failed to process file: {err}");

                send_status(
                    &mut task_progress_tx,
                    task_file_path,
                    Status::ProcessingFailed {
                        error: error_message.to_string(),
                    },
                )
                .await;

                error!("{error_message}");
            } else {
                send_status(
                    &mut task_progress_tx,
                    file_data.file.clone(),
                    Status::ProcessingSuccessful,
                )
                .await;
            }
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}

async fn do_import(
    file_path: String,
    registers: Option<Vec<String>>,
    file_id: i32,
    sped_type: SpedType,
) -> Result<(), Error> {
    let file = tokio::fs::File::open(file_path).await?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Stack of (level, parent_id)
    let mut parent_stack: Vec<(u8, i32)> = Vec::new();

    if matches!(sped_type, SpedType::Efd) {
        register_efd_models();
    } else {
        register_icms_ipi_models();
    };

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

        if !line.starts_with("|") || !line.ends_with("|") {
            continue;
        }

        let reg_code = line[1..5].to_string();

        if let Some(ref regs) = registers {
            if !regs.contains(&reg_code) {
                continue;
            }
        }

        let structure = if matches!(sped_type, SpedType::Efd) {
            utils::file_structure::efd::FILE_STRUCTURE.get(&reg_code.as_str())
        } else {
            utils::file_structure::icms_ipi::FILE_STRUCTURE.get(&reg_code.as_str())
        };

        let level = match structure.is_some() {
            true => structure.unwrap().level,
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

        let parent_id = parent_stack
            .last()
            .map(|&(_, p_id)| p_id)
            .unwrap_or_default();

        let inserted_line = handle_line(&line, parent_id, file_id).await;

        match inserted_line {
            Ok(Some(result)) => parent_stack.push((level, result)),
            Err(error) => {
                println!("Failed to insert line ({reg_code}): {error}");
            }
            _ => {}
        };
    }

    Ok(())
}

async fn create_file_entry(name: String, sped_type: SpedType) -> Result<i32, Error> {
    let sped = match sped_type {
        SpedType::Efd => "efd",
        SpedType::IcmsIpi => "icms_ipi",
    };

    diesel::insert_into(table)
        .values((schema::name.eq(&name), schema::sped_type.eq(&sped)))
        .execute(&mut DB_POOL.get().unwrap())?;

    Ok(sql::<Integer>("SELECT last_insert_rowid()")
        .get_result::<i32>(&mut DB_POOL.get().unwrap())?)
}

pub async fn clean() -> Result<(), anyhow::Error> {
    database::clean().await
}

pub async fn send_status(tx: &mut Option<Sender<Progress>>, file_path: String, status: Status) {
    if let Some(tx) = tx {
        let event = Progress { file_path, status };

        tx.send(event).await.expect("Failed to send status message");
    } else {
        warn!("Notification channel not available");
    }
}
