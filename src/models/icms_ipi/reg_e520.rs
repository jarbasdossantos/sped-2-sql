use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_e520::reg_e520::dsl as schema;
use crate::schemas::reg_e520::reg_e520::table;
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
#[diesel(table_name = crate::schemas::reg_e520::reg_e520::dsl)]
pub struct RegE520 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_sd_ant_ipi: Option<String>,
    pub vl_deb_ipi: Option<String>,
    pub vl_cred_ipi: Option<String>,
    pub vl_od_ipi: Option<String>,
    pub vl_oc_ipi: Option<String>,
    pub vl_sc_ipi: Option<String>,
    pub vl_sd_ipi: Option<String>,
}

#[async_trait]
impl Model for RegE520 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegE520 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_sd_ant_ipi: get_field(&fields, 2),
            vl_deb_ipi: get_field(&fields, 3),
            vl_cred_ipi: get_field(&fields, 4),
            vl_od_ipi: get_field(&fields, 5),
            vl_oc_ipi: get_field(&fields, 6),
            vl_sc_ipi: get_field(&fields, 7),
            vl_sd_ipi: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegE520>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegE520::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegE520::as_select())
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
                    schema::vl_sd_ant_ipi.eq(&self.vl_sd_ant_ipi),
                    schema::vl_deb_ipi.eq(&self.vl_deb_ipi),
                    schema::vl_cred_ipi.eq(&self.vl_cred_ipi),
                    schema::vl_od_ipi.eq(&self.vl_od_ipi),
                    schema::vl_oc_ipi.eq(&self.vl_oc_ipi),
                    schema::vl_sc_ipi.eq(&self.vl_sc_ipi),
                    schema::vl_sd_ipi.eq(&self.vl_sd_ipi),
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
        "RegE520".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegE520 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegE520,
    [
        reg,
        vl_sd_ant_ipi,
        vl_deb_ipi,
        vl_cred_ipi,
        vl_od_ipi,
        vl_oc_ipi,
        vl_sc_ipi,
        vl_sd_ipi
    ]
);
register_model!(RegE520, "e520");
