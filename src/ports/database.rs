use crate::domain::model::Result;
use async_trait::async_trait;
use indexmap::IndexMap;
use std::fmt::Debug;

#[async_trait]
pub trait DatabaseConnection: Debug + Send + Sync {
    async fn insert(&self, table: &str, values: IndexMap<String, String>) -> Result<i32>;
    async fn update(&self, table: &str, id: i32, values: IndexMap<String, String>) -> Result<()>;
    async fn delete(&self, table: &str, id: i32) -> Result<()>;
    async fn find_by_id(&self, table: &str, id: i32) -> Result<Option<IndexMap<String, String>>>;
    async fn find_by_parent(
        &self,
        table: &str,
        file_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Vec<IndexMap<String, String>>>;
    async fn execute_raw(&self, query: &str) -> Result<()>;
    async fn begin_transaction(&self) -> Result<()>;
    async fn commit_transaction(&self) -> Result<()>;
    async fn rollback_transaction(&self) -> Result<()>;
}