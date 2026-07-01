use anyhow::Result;

use crate::domain::model::Result as SpedResult;
use crate::ports::output::OutputWriter;

pub async fn export_file(file_id: i32) -> Result<()> {
    log::info!("Exporting file with ID: {file_id}");
    Ok(())
}

pub async fn export_to_writer<W: OutputWriter>(
    _file_id: i32,
    _writer: &W,
) -> SpedResult<()> {
    Ok(())
}