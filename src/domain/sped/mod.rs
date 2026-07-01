use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use indexmap::IndexMap;

use crate::domain::model::{Result, SpedError};
use crate::ports::database::DatabaseConnection;

#[async_trait]
pub trait SpedImporter: Send + Sync {
    async fn import_file(&self, file_path: &str) -> Result<i32>;
    async fn import_line(&self, line: &str, file_id: i32, parent_id: Option<i32>) -> Result<i32>;
}

pub struct SpedImporterImpl {
    db: Arc<dyn DatabaseConnection>,
    registry: Arc<HashMap<String, Arc<dyn SpedRecordFactory>>>,
}

impl SpedImporterImpl {
    pub fn new(
        db: Arc<dyn DatabaseConnection>,
        registry: Arc<HashMap<String, Arc<dyn SpedRecordFactory>>>,
    ) -> Self {
        Self { db, registry }
    }
}

#[async_trait]
impl SpedImporter for SpedImporterImpl {
    async fn import_file(&self, file_path: &str) -> Result<i32> {
        use indexmap::IndexMap;

        let file_id = self
            .db
            .insert(
                "files",
                IndexMap::from([("name".to_string(), file_path.to_string())]),
            )
            .await?;

        let content = tokio::fs::read_to_string(file_path)
            .await
            .map_err(SpedError::from)?;

        let mut parent_stack: Vec<(u8, i32)> = Vec::new();

        for line in content.lines() {
            if !line.starts_with('|') || !line.ends_with('|') {
                continue;
            }

            let reg_code = &line[1..5];

            while let Some(&(stack_level, _)) = parent_stack.last() {
                if stack_level >= 1 {
                    parent_stack.pop();
                } else {
                    break;
                }
            }

            let parent_id = parent_stack.last().map(|&(_, id)| id);

            match self.import_line(line, file_id, parent_id).await {
                Ok(id) => parent_stack.push((1, id)),
                Err(e) => {
                    log::warn!("Failed to import line ({reg_code}): {e}");
                }
            }
        }

        Ok(file_id)
    }

    async fn import_line(
        &self,
        line: &str,
        file_id: i32,
        parent_id: Option<i32>,
    ) -> Result<i32> {
        let fields: Vec<&str> = line.split('|').collect();
        let reg_code = fields.get(1).ok_or_else(|| {
            SpedError::invalid_format("Missing register code", std::path::PathBuf::new(), 0)
        })?;

        let factory = self.registry.get(*reg_code).ok_or_else(|| {
            SpedError::invalid_format(
                format!("Unknown register: {reg_code}"),
                std::path::PathBuf::new(),
                0,
            )
        })?;

        let record = factory.create(fields, None, parent_id, file_id)?;
        record.save(&self.db).await
    }
}

#[async_trait]
pub trait SpedRecordFactory: Send + Sync {
    fn handle_reg(&self) -> &str;
    fn create(
        &self,
        fields: Vec<&str>,
        id: Option<i32>,
        parent_id: Option<i32>,
        file_id: i32,
    ) -> Result<Box<dyn SpedRecord>>;
}

#[async_trait]
pub trait SpedRecord: Send + Sync {
    async fn save(&self, db: &Arc<dyn DatabaseConnection>) -> Result<i32>;
    fn get_id(&self) -> Option<i32>;
    fn get_file_id(&self) -> Option<i32>;
    fn to_index_map(&self) -> IndexMap<String, String>;
}