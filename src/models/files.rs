use super::traits::Reg;
use crate::database::DB_POOL;
use crate::utils::file_structure::{get_reg_children, FILE_STRUCTURE};
use anyhow::Result;
use sqlx::Row;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Files {
    pub id: Option<i64>,
    pub name: Option<String>,
}

pub(crate) trait FilesTrait: Sized {
    fn new(id: Option<i64>, name: Option<String>) -> Files;
    async fn get(id: i64) -> Pin<Box<dyn Future<Output = Result<Self, anyhow::Error>>>>;
    async fn get_data(id: i64) -> Result<Vec<Box<dyn Reg>>, anyhow::Error>;
}

impl FilesTrait for Files {
    fn new(id: Option<i64>, name: Option<String>) -> Files {
        Files { id, name }
    }

    async fn get(id: i64) -> Pin<Box<dyn Future<Output = Result<Self, anyhow::Error>>>> {
        Box::pin(async move {
            let data = sqlx::query("SELECT ID, NAME FROM files WHERE ID = ?")
                .bind(id)
                .fetch_one(&*DB_POOL)
                .await?;

            let id: i64 = data.try_get("ID")?;
            let name: String = data.try_get("NAME")?;

            Ok(Self::new(Some(id), Some(name)))
        })
    }

    async fn get_data(id: i64) -> Result<Vec<Box<dyn Reg>>, anyhow::Error> {
        async fn fetch_recursive<'a>(
            file_id: i64,
            reg: &str,
            parent_id: Option<i64>,
            all_data: &mut Vec<Box<dyn Reg>>,
        ) -> Result<(), anyhow::Error> {
            let structure = match FILE_STRUCTURE.get(reg) {
                Some(s) => s,
                None => return Ok(()),
            };

            let model = match structure.load_model {
                Some(m) => m,
                None => return Ok(()),
            };

            let rows = model(file_id, parent_id).await.expect("Failed to load model data");
            let children = get_reg_children();

            for row in rows {
                let id = row
                    .values()
                    .get("id")
                    .and_then(|v| v.as_ref().unwrap().parse::<i64>().ok());

                all_data.push(row);

                if let Some(child_regs) = children.get(reg) {
                    for child_reg in child_regs {
                        Box::pin(fetch_recursive(file_id, child_reg, id, all_data)).await?;
                    }
                }
            }

            Ok(())
        }

        let file = Self::get(id).await.await?;
        let mut all_data: Vec<Box<dyn Reg>> = Vec::new();

        Box::pin(fetch_recursive(file.id.unwrap(), "0000", None, &mut all_data)).await?;

        Ok(all_data)
    }
}
