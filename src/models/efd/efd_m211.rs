#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m211::efd_m211::dsl as schema;
use crate::schemas::efd_m211::efd_m211::table;
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
#[diesel(table_name = crate::schemas::efd_m211::efd_m211::dsl)]
pub struct EfdM211 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_tip_coop: Option<String>,
    pub vl_bc_cont_ant_exc_coop: Option<String>,
    pub vl_exc_coop_ger: Option<String>,
    pub vl_exc_esp_coop: Option<String>,
    pub vl_bc_cont: Option<String>,
}

#[async_trait]
impl Model for EfdM211 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM211 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_tip_coop: get_field(&fields, 2),
            vl_bc_cont_ant_exc_coop: get_field(&fields, 3),
            vl_exc_coop_ger: get_field(&fields, 4),
            vl_exc_esp_coop: get_field(&fields, 5),
            vl_bc_cont: get_field(&fields, 6),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM211>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM211::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM211::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::ind_tip_coop.eq(&self.ind_tip_coop),
                    schema::vl_bc_cont_ant_exc_coop.eq(&self.vl_bc_cont_ant_exc_coop),
                    schema::vl_exc_coop_ger.eq(&self.vl_exc_coop_ger),
                    schema::vl_exc_esp_coop.eq(&self.vl_exc_esp_coop),
                    schema::vl_bc_cont.eq(&self.vl_bc_cont),
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
        "EfdM211".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM211 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    EfdM211,
    [
        reg,
        ind_tip_coop,
        vl_bc_cont_ant_exc_coop,
        vl_exc_coop_ger,
        vl_exc_esp_coop,
        vl_bc_cont
    ]
);
register_model!(EfdM211, "m211");
