use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c180::efd_c180::dsl as schema;
use crate::schemas::efd_c180::efd_c180::table;
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
#[diesel(table_name = crate::schemas::efd_c180::efd_c180::dsl)]
pub struct EfdC180 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub dt_doc_ini: Option<String>,
    pub dt_doc_fin: Option<String>,
    pub cod_item: Option<String>,
    pub cod_ncm: Option<String>,
    pub ex_ipi: Option<String>,
    pub vl_tot_item: Option<String>,
}

#[async_trait]
impl Model for EfdC180 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC180 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            dt_doc_ini: get_field(&fields, 3),
            dt_doc_fin: get_field(&fields, 4),
            cod_item: get_field(&fields, 5),
            cod_ncm: get_field(&fields, 6),
            ex_ipi: get_field(&fields, 7),
            vl_tot_item: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC180>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdC180::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::dt_doc_ini.eq(&self.dt_doc_ini),
                    schema::dt_doc_fin.eq(&self.dt_doc_fin),
                    schema::cod_item.eq(&self.cod_item),
                    schema::cod_ncm.eq(&self.cod_ncm),
                    schema::ex_ipi.eq(&self.ex_ipi),
                    schema::vl_tot_item.eq(&self.vl_tot_item),
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
        "EfdC180".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC180 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC180, [reg, cod_mod, dt_doc_ini, dt_doc_fin, cod_item, cod_ncm, ex_ipi, vl_tot_item]);
register_model!(EfdC180, "c180");
