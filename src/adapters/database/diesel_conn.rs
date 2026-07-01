use crate::domain::model::{Result, SpedError};
use crate::ports::database::DatabaseConnection;
use async_trait::async_trait;
use indexmap::IndexMap;
use std::fmt::Debug;

pub struct DieselConnection;

impl DieselConnection {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DieselConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for DieselConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DieselConnection").finish()
    }
}

fn unimplemented() -> SpedError {
    SpedError::Configuration(
        "DieselConnection method not yet implemented".to_string(),
    )
}

#[async_trait]
impl DatabaseConnection for DieselConnection {
    async fn insert(&self, _table: &str, _values: IndexMap<String, String>) -> Result<i32> {
        Err(unimplemented())
    }

    async fn update(
        &self,
        _table: &str,
        _id: i32,
        _values: IndexMap<String, String>,
    ) -> Result<()> {
        Err(unimplemented())
    }

    async fn delete(&self, _table: &str, _id: i32) -> Result<()> {
        Err(unimplemented())
    }

    async fn find_by_id(
        &self,
        _table: &str,
        _id: i32,
    ) -> Result<Option<IndexMap<String, String>>> {
        Err(unimplemented())
    }

    async fn find_by_parent(
        &self,
        _table: &str,
        _file_id: i32,
        _parent_id: Option<i32>,
    ) -> Result<Vec<IndexMap<String, String>>> {
        Err(unimplemented())
    }

    async fn execute_raw(&self, _query: &str) -> Result<()> {
        Err(unimplemented())
    }

    async fn begin_transaction(&self) -> Result<()> {
        Err(unimplemented())
    }

    async fn commit_transaction(&self) -> Result<()> {
        Err(unimplemented())
    }

    async fn rollback_transaction(&self) -> Result<()> {
        Err(unimplemented())
    }
}