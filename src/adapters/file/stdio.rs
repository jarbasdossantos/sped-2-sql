use crate::domain::model::Result;
use crate::ports::file::{FileReader, FileWriter};
use async_trait::async_trait;
use std::fmt::Debug;

pub struct StdioFileAdapter;

impl Debug for StdioFileAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdioFileAdapter").finish()
    }
}

#[async_trait]
impl FileReader for StdioFileAdapter {
    async fn read_file(&self, path: &str) -> Result<String> {
        let content = tokio::fs::read_to_string(path).await?;
        Ok(content)
    }

    async fn read_lines(&self, path: &str) -> Result<Vec<String>> {
        let content = self.read_file(path).await?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Ok(lines)
    }
}

#[async_trait]
impl FileWriter for StdioFileAdapter {
    async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}