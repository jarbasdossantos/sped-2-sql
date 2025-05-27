use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_0205::efd_0205::dsl as schema;
use crate::schemas::efd_0205::efd_0205::table;
use crate::{impl_display_fields, register_model};
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use diesel::{QueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::efd_0205::efd_0205::dsl)]
pub struct Efd0205 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub descr_ant_item: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fim: Option<String>,
    pub cod_ant_item: Option<String>,
}

#[async_trait]
impl Model for Efd0205 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd0205 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            descr_ant_item: get_field(&fields, 2),
            dt_ini: get_field(&fields, 3),
            dt_fim: get_field(&fields, 4),
            cod_ant_item: get_field(&fields, 5),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd0205>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(Efd0205::as_select())
            .load(&mut DB_POOL
                .get().unwrap())?)
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::descr_ant_item.eq(&self.descr_ant_item),
                    schema::dt_ini.eq(&self.dt_ini),
                    schema::dt_fim.eq(&self.dt_fim),
                    schema::cod_ant_item.eq(&self.cod_ant_item),
                ))
                .execute(&mut DB_POOL.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Efd0205".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd0205 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd0205, [reg, descr_ant_item, dt_ini, dt_fim, cod_ant_item]);
register_model!(Efd0205, "0205");
