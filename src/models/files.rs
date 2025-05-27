use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::FilesModel;
use crate::schemas::files::files::dsl as schema;
use crate::utils::file_structure::{get_reg_children, FILE_STRUCTURE};
use crate::Export;
use anyhow::Result;

use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Selectable};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::files::files)]
#[allow(dead_code)]
pub struct File {
    pub id: i32,
    pub name: Option<String>,
}

#[async_trait]
impl FilesModel for File {
    async fn get(file_id: i32) -> Result<Box<File>, anyhow::Error> {
        match crate::schemas::files::files::table
            .filter(schema::id.eq(&file_id))
            .first::<File>(&mut DB_POOL.get()?)
        {
            Ok(file) => Ok(Box::new(file)),
            Err(err) => Err(anyhow::Error::from(err)),
        }
    }

    async fn get_data(file_data: Export) -> Result<Vec<Box<dyn Model>>, anyhow::Error> {
        let mut all_data: Vec<Box<dyn Model>> = Vec::new();

        async fn fetch_iterative(
            initial_file_id: i32,
            initial_register: String,
            initial_parent_id: Option<i32>,
            initial_registers: Option<Vec<String>>,
        ) -> Result<Vec<Box<dyn Model>>, anyhow::Error> {
            let mut data: Vec<Box<dyn Model>> = Vec::new();
            let mut stack: Vec<(i32, String, Option<i32>, Option<Vec<String>>)> = Vec::new();

            stack.push((
                initial_file_id,
                initial_register,
                initial_parent_id,
                initial_registers.clone(),
            ));

            let children_map = get_reg_children();

            while let Some((file_id, register, parent_id, registers_opt)) = stack.pop() {
                if let Some(ref regs_filter) = registers_opt {
                    if !regs_filter.contains(&register) {
                        continue;
                    }
                }

                let structure = match FILE_STRUCTURE.get(register.as_str()) {
                    Some(s) => s,
                    None => continue,
                };

                let load = match &structure.load_model {
                    Some(load_fn) => load_fn,
                    None => continue,
                };

                let rows = match load(file_id, parent_id).await {
                    Ok(r) => r,
                    Err(e) => return Err(anyhow::anyhow!("{}", e)),
                };

                for row_box in rows {
                    let model_file_id = row_box.get_file_id().unwrap_or(file_id);
                    let model_id = row_box.get_id();

                    data.push(row_box);

                    if let Some(child_regs_for_current) = children_map.get(register.as_str()) {
                        for child_reg_str in child_regs_for_current.iter().rev() {
                            stack.push((
                                model_file_id,
                                child_reg_str.to_string(),
                                model_id,
                                registers_opt.clone(),
                            ));
                        }
                    }
                }
            }

            Ok(data)
        }

        let file = Self::get(file_data.id).await?;

        if let Some(ref regs) = file_data.registers {
            if let Some(ref reg) = regs.first() {
                fetch_iterative(file.id, reg.to_string(), None, file_data.registers.clone()).await
            } else {
                Ok(Vec::new())
            }
        } else {
            for (code, reg) in FILE_STRUCTURE.iter() {
                if reg.level <= 1 {
                    for data in
                        fetch_iterative(file.id, code.to_string(), Some(0), file_data.registers.clone())
                            .await
                            .unwrap()
                    {
                        all_data.push(data);
                    }
                }
            }

            Ok(all_data)
        }
    }
}
