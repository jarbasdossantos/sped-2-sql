#[allow(clippy::all)]
use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::FilesModel;
use crate::schemas::files::dsl as schema;
use crate::utils::file_structure::{efd::FILE_STRUCTURE, get_reg_children};
use crate::{utils, ExportFile, SpedType};
use anyhow::Result;

use crate::schemas::files::table;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::files)]
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

        async fn fetch_recursive(
            current_file_id: i32,
            current_register: String,
            current_parent_id: Option<i32>,
            registers_filter: &Option<Vec<String>>,
            sped_type: SpedType,
            all_data_accumulator: &mut Vec<Box<dyn Model>>,
            children_map: &std::collections::HashMap<String, Vec<String>>,
        ) -> Result<(), anyhow::Error> {
            if let Some(ref regs_filter) = registers_filter {
                if !regs_filter.contains(&current_register) {
                    return Ok(());
                }
            }

            let sped_structure = if matches!(sped_type, SpedType::Efd) {
                FILE_STRUCTURE.get(current_register.as_str())
            } else {
                utils::file_structure::icms_ipi::FILE_STRUCTURE.get(current_register.as_str())
            };

            let structure = match sped_structure {
                Some(s) => s,
                None => return Ok(()),
            };

            let load = match &structure.load_model {
                Some(load_fn) => load_fn,
                None => {
                    log::warn!("No load function for register {}", current_register);
                    return Ok(());
                }
            };

            let rows = match load(current_file_id, current_parent_id).await {
                Ok(r) => r,
                Err(e) => return Err(anyhow::anyhow!("{}", e)),
            };

            for row_box in rows {
                let model_file_id = row_box.get_file_id().unwrap_or(current_file_id);
                let model_id = row_box.get_id();

                all_data_accumulator.push(row_box);

                if let Some(child_regs_for_current) = children_map.get(current_register.as_str()) {
                    for child_reg_str in child_regs_for_current {
                        Box::pin(fetch_recursive(
                            model_file_id,
                            child_reg_str.to_string(),
                            model_id,
                            registers_filter,
                            sped_type,
                            all_data_accumulator,
                            children_map,
                        ))
                        .await?;
                    }
                }
            }
            Ok(())
        }

        let file = Self::get_file(file_data.id, Some(file_data.sped_type)).await?;
        let children_map = get_reg_children(file_data.sped_type);

        if let Some(ref regs) = file_data.registers {
            if let Some(ref reg) = regs.first() {
                fetch_recursive(
                    file.id,
                    reg.to_string(),
                    None,
                    &file_data.registers,
                    file_data.sped_type,
                    &mut all_data,
                    &children_map,
                )
                .await?;
            }
        } else {
            for (code, reg_info) in FILE_STRUCTURE.iter() {
                if reg_info.level <= 1 {
                    fetch_recursive(
                        file.id,
                        code.to_string(),
                        Some(0), // Assuming Some(0) is the correct initial parent_id for root level
                        &file_data.registers,
                        file_data.sped_type,
                        &mut all_data,
                        &children_map,
                    )
                    .await?;
                }
            }
        }
        Ok(all_data)
    }
}
