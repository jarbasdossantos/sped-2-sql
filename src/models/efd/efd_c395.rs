use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c395::efd_c395::dsl as schema;
use crate::schemas::efd_c395::efd_c395::table;
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
#[diesel(table_name = crate::schemas::efd_c395::efd_c395::dsl)]
pub struct EfdC395 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_part: Option<String>,
    pub ser: Option<String>,
    pub sub_ser: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_doc: Option<String>,
}

#[async_trait]
impl Model for EfdC395 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC395 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    cod_mod: get_field(&fields, 2),
        cod_part: get_field(&fields, 3),
        ser: get_field(&fields, 4),
        sub_ser: get_field(&fields, 5),
        num_doc: get_field(&fields, 6),
        dt_doc: get_field(&fields, 7),
        vl_doc: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC395>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdC395::as_select())
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
schema::cod_part.eq(&self.cod_part),
schema::ser.eq(&self.ser),
schema::sub_ser.eq(&self.sub_ser),
schema::num_doc.eq(&self.num_doc),
schema::dt_doc.eq(&self.dt_doc),
schema::vl_doc.eq(&self.vl_doc),
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
        "EfdC395".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC395 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC395, [reg, cod_mod, cod_part, ser, sub_ser, num_doc, dt_doc, vl_doc]);
register_model!(EfdC395, "c395");
