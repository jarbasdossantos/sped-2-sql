use std::path::Path;

use anyhow::Result as AnyhowResult;

use crate::domain::model::Result as SpedResult;
use crate::domain::sped::SpedImporter;
use crate::ports::file::FileReader;

pub async fn import_file(file_path: &str) -> AnyhowResult<()> {
    use crate::adapters::file::stdio::StdioFileAdapter;

    let path = Path::new(file_path);
    if !path.exists() {
        anyhow::bail!("File not found: {file_path}");
    }

    let reader = StdioFileAdapter;
    let content = reader
        .read_file(file_path)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let line_count = content.lines().count();
    log::info!("Processing {line_count} lines from {file_path}");

    Ok(())
}

pub async fn import_with_importer<I: SpedImporter + ?Sized>(
    importer: &I,
    file_path: &str,
) -> SpedResult<()> {
    importer.import_file(file_path).await?;
    Ok(())
}