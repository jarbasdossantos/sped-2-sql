use crate::domain::model::Result;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait OutputWriter: Debug + Send + Sync {
    async fn write_output(&self, data: &str) -> Result<()>;
    async fn write_error(&self, error: &str) -> Result<()>;
}