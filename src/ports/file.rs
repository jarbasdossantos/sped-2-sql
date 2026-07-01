use crate::domain::model::Result;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait FileReader: Debug + Send + Sync {
    async fn read_file(&self, path: &str) -> Result<String>;
    async fn read_lines(&self, path: &str) -> Result<Vec<String>>;
}

#[async_trait]
pub trait FileWriter: Debug + Send + Sync {
    async fn write_file(&self, path: &str, content: &str) -> Result<()>;
}