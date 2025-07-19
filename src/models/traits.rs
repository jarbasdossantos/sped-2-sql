use crate::models::files::File;
use crate::{ExportFile, SpedType};
use anyhow::Result;
use async_trait::async_trait;
use diesel::result::Error;
use indexmap::IndexMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

#[async_trait]
pub trait Model: Debug + Display + Send + Sync {
    fn new(fields: Vec<&str>, id: Option<i32>, parent_id: Option<i32>, file_id: i32) -> Self
    where
        Self: Sized;

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut diesel::sqlite::SqliteConnection) -> Result<Vec<Self>, Error>
    where
        Self: Sized;

    fn save<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output=std::result::Result<i32, Error>> + Send + 'a>>;

    async fn save_with_retry(&self) -> Result<i32, Error> {
        let mut attempts = 0;
        let max_attempts = 5;
        let delay_ms = 50;

        loop {
            match self.save().await {
                Ok(id) => return Ok(id),
                Err(e) => {
                    if e.to_string().contains("database is locked") && attempts < max_attempts - 1 {
                        attempts += 1;
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn get_id(&self) -> Option<i32>;
    fn get_file_id(&self) -> Option<i32>;
    fn get_entity_name(&self) -> String;

    fn get_display_fields(&self) -> Vec<(String, String)>;

    fn display_format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = self.get_display_fields();

        let string = fields
            .into_iter()
            .map(|(_, value)| value)
            .collect::<Vec<String>>()
            .join("|");

        let string = format!("|{string}|");

        writeln!(f, "{string}")?;

        Ok(())
    }

    fn data(&self) -> Result<IndexMap<String, String>, Error> {
        Ok(self.get_display_fields().into_iter().collect())
    }
}

#[async_trait]
pub trait FilesModel: Send + Sync {
    async fn get_file(file_id: i32, sped_type: Option<SpedType>) -> Result<File, anyhow::Error>;
    async fn stream_file_data(file: ExportFile) -> Result<tokio::sync::mpsc::Receiver<Box<dyn Model>>, anyhow::Error>;
}

pub(crate) trait ModelFactory: Debug {
    fn handle_reg(&self) -> &'static str;
    fn create_model(
        &self,
        fields: Vec<&str>,
        id: Option<i32>,
        parent_id: Option<i32>,
        file_id: i32,
    ) -> Box<dyn Model>;
}
