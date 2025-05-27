use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c400::efd_c400::dsl as schema;
use crate::schemas::efd_c400::efd_c400::table;
use crate::{impl_display_fields, register_model};
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use diesel::{QueryDsl, SelectableHelper};
use serde::{Serialize, Deserialize};
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::efd_c400::efd_c400::dsl)]
pub struct EfdC400 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub ecf_mod: Option<String>,
    pub ecf_fab: Option<String>,
    pub ecf_cx: Option<String>,
}

#[async_trait]
impl Model for EfdC400 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC400 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    cod_mod: get_field(&fields, 2),
        ecf_mod: get_field(&fields, 3),
        ecf_fab: get_field(&fields, 4),
        ecf_cx: get_field(&fields, 5),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC400>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdC400::as_select())
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
schema::ecf_mod.eq(&self.ecf_mod),
schema::ecf_fab.eq(&self.ecf_fab),
schema::ecf_cx.eq(&self.ecf_cx),
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
        "EfdC400".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC400 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC400, [reg, cod_mod, ecf_mod, ecf_fab, ecf_cx]);
register_model!(EfdC400, "c400");
