use std::path::PathBuf;

use crate::adapters::cli::commands::{Cli, Command, SpedTypeArg};
use crate::adapters::file::stdio::StdioFileAdapter;
use crate::ports::file::FileReader;
use crate::{ExportFile, ImportFiles, ImportFilesData, SpedType};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&cli.log_level),
    )
    .try_init();

    let database_url = resolve_database_url(&cli)?;

    crate::database::initialize_pool(database_url).map_err(|e| anyhow::anyhow!("{e}"))?;
    crate::database::migrate().await;

    match cli.command {
        Command::Import {
            files,
            sped_type,
            registers,
        } => run_import(files, sped_type, registers).await,
        Command::Export {
            file_id,
            sped_type,
            registers,
            output,
        } => run_export(file_id, sped_type, registers, output).await,
        Command::Migrate => {
            log::info!("Migrations applied");
            Ok(())
        }
        Command::Validate {
            file,
            sped_type,
        } => run_validate(file, sped_type).await,
    }
}

fn resolve_database_url(cli: &Cli) -> anyhow::Result<String> {
    if cli.memory_db {
        return Ok(":memory:".to_string());
    }
    if let Some(url) = &cli.database_url {
        return Ok(url.clone());
    }
    Err(anyhow::anyhow!(
        "Database URL required: pass --database-url <path>, set DATABASE_URL, or use --memory-db"
    ))
}

fn map_sped_type(arg: SpedTypeArg) -> SpedType {
    match arg {
        SpedTypeArg::Efd => SpedType::Efd,
        SpedTypeArg::IcmsIpi => SpedType::IcmsIpi,
    }
}

fn parse_registers(registers: Option<String>) -> Option<Vec<String>> {
    registers.map(|r| {
        r.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    })
}

async fn run_import(
    files: Vec<PathBuf>,
    sped_type: SpedTypeArg,
    registers: Option<String>,
) -> anyhow::Result<()> {
    let registers = parse_registers(registers);

    let files_data = files
        .into_iter()
        .enumerate()
        .map(|(i, path)| ImportFilesData {
            id: i as i32 + 1,
            file: path.to_string_lossy().to_string(),
            registers: registers.clone(),
            sped_type: map_sped_type(sped_type),
        })
        .collect();

    let data = ImportFiles {
        files: files_data,
        progress_tx: None,
    };

    crate::import(data).await?;
    log::info!("Import finished");
    Ok(())
}

async fn run_export(
    file_id: i32,
    sped_type: SpedTypeArg,
    registers: Option<String>,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let registers = parse_registers(registers);

    let mut receiver = crate::export(ExportFile {
        file_id,
        registers,
        sped_type: map_sped_type(sped_type),
    })
    .await?;

    match output {
        Some(path) => {
            let content = collect_models(&mut receiver).await?;
            tokio::fs::write(path, content).await?;
        }
        None => {
            use tokio::io::AsyncWriteExt;
            let mut writer = tokio::io::stdout();
            while let Some(model) = receiver.recv().await {
                let line = model.to_string();
                writer.write_all(line.as_bytes()).await?;
            }
        }
    }

    log::info!("Export finished");
    Ok(())
}

async fn collect_models(
    receiver: &mut tokio::sync::mpsc::Receiver<Box<dyn crate::models::traits::Model>>,
) -> anyhow::Result<String> {
    let mut buf = String::new();
    while let Some(model) = receiver.recv().await {
        buf.push_str(&model.to_string());
    }
    Ok(buf)
}

async fn run_validate(file: PathBuf, sped_type: SpedTypeArg) -> anyhow::Result<()> {
    if !file.exists() {
        anyhow::bail!("File not found: {}", file.display());
    }

    if file.extension().and_then(|e| e.to_str()) != Some("txt") {
        anyhow::bail!("Only .txt files are supported");
    }

    let reader = StdioFileAdapter;
    let content = reader
        .read_file(&file.to_string_lossy())
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut valid = 0;
    let mut invalid = 0;

    for (i, line) in content.lines().enumerate() {
        if line.starts_with('|') && line.ends_with('|') {
            valid += 1;
        } else if !line.is_empty() {
            invalid += 1;
            log::warn!("Invalid line {}: does not start/end with '|'", i + 1);
        }
    }

    log::info!(
        "Validation ({}): {valid} valid lines, {invalid} invalid lines",
        match sped_type {
            SpedTypeArg::Efd => "EFD",
            SpedTypeArg::IcmsIpi => "ICMS-IPI",
        }
    );

    if invalid > 0 {
        anyhow::bail!("File contains {invalid} invalid lines");
    }
    Ok(())
}