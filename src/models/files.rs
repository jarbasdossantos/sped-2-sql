use crate::database::DB_POOL;
use crate::utils::file_structure::{get_reg_children, FILE_STRUCTURE};
use anyhow::Result;
use sqlx::Row;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use super::traits::Reg;

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
        let file = Self::get(id).await.await?;
        let mut prev_level = 0;
        let mut parent_stack: Vec<(u8, i64)> = Vec::new();
        let mut to_skip: HashSet<&'static str> = HashSet::new();

        let mut all_data: Vec<Box<dyn Reg>> = Vec::new();

        let children = get_reg_children();
        let plain_children: Vec<String> = children
            .iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<String>>();

        for (reg, structure) in FILE_STRUCTURE.iter() {
            // If the register was processed by children logic, skip it
            if to_skip.contains(reg) && plain_children.contains(&reg.to_string()) {
                continue;
            }

            if structure.level > prev_level {
                prev_level = structure.level;
            }

            while let Some(&(stack_level, _)) = parent_stack.last() {
                if stack_level >= structure.level {
                    parent_stack.pop();
                } else {
                    break;
                }
            }

            let parent_id = parent_stack.last().map(|&(_, id)| id);

            let data_vec = match structure.load_model {
                Some(model) => {
                    model(file.id.unwrap(), parent_id).map_err(|e| anyhow::anyhow!("{}", e))?
                }
                None => {
                    // eprintln!("No model for {}", reg);
                    continue;
                }
            };

            for data in data_vec {
                let id = data
                    .values()
                    .get("id")
                    .and_then(|v| v.as_ref().unwrap().parse::<i64>().ok());

                all_data.push(data);

                let child_regs = children.get(reg);

                if child_regs.is_some() && reg != &"0000" {
                    for child_reg in child_regs.unwrap() {
                        to_skip.insert(child_reg);

                        let child_model = match FILE_STRUCTURE.get(child_reg).unwrap().load_model {
                            model => model,
                        };

                        if child_model.is_none() {
                            continue;
                        }

                        let child_data_vec = child_model.unwrap()(file.id.unwrap(), id).unwrap();

                        for child_data in child_data_vec {
                            all_data.push(child_data);
                        }
                    }
                }

                parent_stack.push((structure.level, id.unwrap()));
            }
        }

        Ok(all_data)
    }
}
