#[allow(clippy::all)]
use super::traits::Model;
use crate::database::DB_POOL;
use crate::models::traits::FilesModel;
use crate::schemas::files::dsl as schema;
use crate::utils::file_structure::{efd::FILE_STRUCTURE, get_reg_children};
use crate::{utils, ExportFile, SpedType};
use anyhow::Result;
use tokio::sync::mpsc;
use diesel::sqlite::SqliteConnection;

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
        let mut conn = DB_POOL.lock().await.get().unwrap();

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

    async fn stream_file_data(
        file_data: ExportFile,
    ) -> Result<mpsc::Receiver<Box<dyn Model>>, anyhow::Error> {
        let (tx, rx) = mpsc::channel(100);

        let file = Self::get_file(file_data.file_id, Some(file_data.sped_type)).await?;
        let children_map = get_reg_children(file_data.sped_type);

        let registers = file_data.registers.clone();
        let sped_type = file_data.sped_type;

        let mut conn = DB_POOL.lock().await.get()?;

        tokio::task::spawn_blocking(move || {
            fn fetch_recursive_stream(
                current_file_id: i32,
                current_register: String,
                current_parent_id: Option<i32>,
                registers_filter: &Option<Vec<String>>,
                sped_type: SpedType,
                tx: &mpsc::Sender<Box<dyn Model>>,
                children_map: &std::collections::HashMap<String, Vec<String>>,
                conn: &mut SqliteConnection,
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
                    None => return Ok(()),
                };

                let rows = load(current_file_id, current_parent_id, conn)?;

                for row_box in rows {
                    let model_file_id = row_box.get_file_id().unwrap_or(current_file_id);
                    let model_id = row_box.get_id();

                    if tx.blocking_send(row_box).is_err() {
                        return Ok(());
                    }

                    if let Some(child_regs_for_current) =
                        children_map.get(current_register.as_str())
                    {
                        for child_reg_str in child_regs_for_current {
                            fetch_recursive_stream(
                                model_file_id,
                                child_reg_str.to_string(),
                                model_id,
                                registers_filter,
                                sped_type,
                                tx,
                                children_map,
                                conn,
                            )?;
                        }
                    }
                }
                Ok(())
            }

            if let Some(ref regs) = registers {
                if let Some(ref reg) = regs.first() {
                    let _ = fetch_recursive_stream(
                        file.id,
                        reg.to_string(),
                        None,
                        &registers,
                        sped_type,
                        &tx,
                        &children_map,
                        &mut conn,
                    );
                }
            } else {
                let structure = if matches!(sped_type, SpedType::Efd) {
                    &FILE_STRUCTURE
                } else {
                    &utils::file_structure::icms_ipi::FILE_STRUCTURE
                };
                for (code, reg_info) in structure.iter() {
                    if reg_info.level <= 1 {
                        let _ = fetch_recursive_stream(
                            file.id,
                            code.to_string(),
                            Some(0),
                            &None,
                            sped_type,
                            &tx,
                            &children_map,
                            &mut conn,
                        );
                    }
                }
            }
        });

        Ok(rx)
    }
}
