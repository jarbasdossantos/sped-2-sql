mod database;
pub mod macros;
pub mod models;
mod schemas;
mod sped;
mod utils;

use crate::models::files::File;
use crate::models::registry::{register_efd_models, register_icms_ipi_models};
use crate::models::traits::FilesModel;
use crate::schemas::files::files::dsl as schema;
use crate::schemas::files::files::table;
use anyhow::{Error, Result};
use database::DB_POOL;
use diesel::dsl::sql;
use diesel::sql_types::Integer;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use encoding_rs::UTF_8;
use futures::future::join_all;
use models::traits::Model;
use sped::handle_line;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SpedType {
    Efd,
    IcmsIpi,
}

pub struct LoadData {
    /// File path
    pub file: String,
    /// Registers to import
    pub registers: Option<Vec<String>>,
    pub sped_type: SpedType,
}

#[derive(Debug, Clone)]
pub struct Export {
    /// File id
    pub id: i32,
    /// Registers to export
    pub registers: Option<Vec<String>>,
    /// Sped type
    pub sped_type: SpedType,
}

pub struct Load {
    pub files: Vec<LoadData>,
}

pub async fn load(data: Load) -> Result<(), Error> {
    let processing: task::JoinHandle<std::result::Result<(), Error>> =
        task::spawn(async move { import_files(data.files).await });

    processing.await?
}

pub async fn export(data: Export) -> Result<Vec<Box<dyn Model>>, Error> {
    if matches!(data.sped_type, SpedType::Efd) {
        register_efd_models();
    } else {
        register_icms_ipi_models();
    };

    let file_data = File::get_data(data).await?;

    Ok(file_data)
}

async fn import_files(data: Vec<LoadData>) -> Result<(), Error> {
    // database::migrate().await;

    // let mut tasks = vec![];

    if data.is_empty() {
        return Ok(());
    }

    for data in data {
        let extension = data
            .file
            .split('.')
            .next_back()
            .map(|s| s.to_string())
            .unwrap();

        let file_name = data
            .file
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap_or("")
            .to_string();

        let file_id = match create_file_entry(file_name.clone()).await {
            Ok(file_id) => file_id,
            Err(err) => {
                println!("Failed to create file entry: {}", err);
                continue;
            }
        };

        if extension != "txt" {
            continue;
        }

        let file_clone = data.file.clone();

        if let Err(err) = import(file_clone, data.registers, file_id, data.sped_type).await {
            let message = format!("Failed to process file: {}", err);
            eprintln!("{message}");
        }

        // let task = task::spawn(async move {
        //     if let Err(err) = import(file_clone, data.registers, file_id, data.sped_type).await {
        //         let message = format!("Failed to process file: {}", err);
        //         eprintln!("{message}");
        //     }
        // });
        //
        // tasks.push(task);
    }

    // join_all(tasks).await;

    Ok(())
}

async fn import(
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

        if !line.ends_with("|") {
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

        let inserted_line = handle_line(&line, parent_id, file_id, sped_type).await;

        match inserted_line {
            Ok(Some(result)) => parent_stack.push((level, result)),
            Err(error) => {
                println!("Failed to insert line ({}): {}", reg_code, error);
            }
            _ => {}
        };
    }

    Ok(())
}

async fn create_file_entry(name: String) -> Result<i32, Error> {
    diesel::insert_into(table)
        .values((schema::name.eq(&name),))
        .execute(&mut DB_POOL.get().unwrap())?;

    Ok(sql::<Integer>("SELECT last_insert_rowid()")
        .get_result::<i32>(&mut DB_POOL.get().unwrap())?)
}

pub async fn clean() -> Result<(), anyhow::Error> {
    Ok(database::clean().await?)
}
