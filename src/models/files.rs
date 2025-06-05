#[allow(clippy::all)]
use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::FilesModel;
use crate::schemas::files::files::dsl as schema;
use crate::utils::file_structure::{efd::FILE_STRUCTURE, get_reg_children};
use crate::{utils, ExportFile, SpedType};
use anyhow::Result;

use crate::schemas::files::files::table;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::files::files)]
#[allow(dead_code)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub sped_type: String,
}

#[async_trait]
impl FilesModel for File {
    async fn get_file(file_id: i32, sped_type: Option<SpedType>) -> Result<File, anyhow::Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(sped) = sped_type {
            let sped = match sped {
                SpedType::Efd => "efd",
                SpedType::IcmsIpi => "icms_ipi",
            };

            Ok(table
                .filter(schema::sped_type.eq(&sped))
                .filter(schema::id.eq(&file_id))
                .select(File::as_select())
                .first::<File>(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::id.eq(&file_id))
                .select(File::as_select())
                .first::<File>(&mut conn)?)
        }
    }

    async fn get_file_data(file_data: ExportFile) -> Result<Vec<Box<dyn Model>>, anyhow::Error> {
        let mut all_data: Vec<Box<dyn Model>> = Vec::new();

        async fn fetch(
            initial_file_id: i32,
            initial_register: String,
            initial_parent_id: Option<i32>,
            initial_registers: Option<Vec<String>>,
            sped_type: SpedType,
        ) -> Result<Vec<Box<dyn Model>>, anyhow::Error> {
            let mut data: Vec<Box<dyn Model>> = Vec::new();
            let mut stack: Vec<(i32, String, Option<i32>, Option<Vec<String>>)> = Vec::new();

            stack.push((
                initial_file_id,
                initial_register,
                initial_parent_id,
                initial_registers.clone(),
            ));

            let children_map = get_reg_children(sped_type);

            while let Some((file_id, register, parent_id, registers_opt)) = stack.pop() {
                if let Some(ref regs_filter) = registers_opt {
                    if !regs_filter.contains(&register) {
                        continue;
                    }
                }

                let sped_structure = if matches!(sped_type, SpedType::Efd) {
                    FILE_STRUCTURE.get(register.as_str())
                } else {
                    utils::file_structure::icms_ipi::FILE_STRUCTURE.get(register.as_str())
                };

                let structure = match sped_structure {
                    Some(s) => s,
                    None => continue,
                };

                let load = match &structure.load_model {
                    Some(load_fn) => load_fn,
                    None => {
                        log::warn!("No load function for register {register}");
                        continue;
                    }
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

        let file = Self::get_file(file_data.id, Some(file_data.sped_type)).await?;

        if let Some(ref regs) = file_data.registers {
            if let Some(ref reg) = regs.first() {
                for data in fetch(
                    file.id,
                    reg.to_string(),
                    None,
                    file_data.registers.clone(),
                    file_data.sped_type,
                )
                .await
                .unwrap()
                {
                    all_data.push(data);
                }
            }

            Ok(all_data)
        } else {
            for (code, reg) in FILE_STRUCTURE.iter() {
                if reg.level <= 1 {
                    for data in fetch(
                        file.id,
                        code.to_string(),
                        Some(0),
                        file_data.registers.clone(),
                        file_data.sped_type,
                    )
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
