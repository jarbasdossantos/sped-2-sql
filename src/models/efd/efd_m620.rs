use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m620::dsl as schema;
use crate::schemas::efd_m620::table;
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
#[diesel(table_name = crate::schemas::efd_m620::dsl)]
pub struct EfdM620 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_aj: Option<String>,
    pub vl_aj: Option<String>,
    pub cod_aj: Option<String>,
    pub num_doc: Option<String>,
    pub descr_aj: Option<String>,
    pub dt_ref: Option<String>,
}

#[async_trait]
impl Model for EfdM620 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM620 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_aj: get_field(&fields, 2),
            vl_aj: get_field(&fields, 3),
            cod_aj: get_field(&fields, 4),
            num_doc: get_field(&fields, 5),
            descr_aj: get_field(&fields, 6),
            dt_ref: get_field(&fields, 7),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM620>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM620::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM620::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = DB_POOL.lock().await.get().unwrap();

            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::ind_aj.eq(&self.ind_aj),
                    schema::vl_aj.eq(&self.vl_aj),
                    schema::cod_aj.eq(&self.cod_aj),
                    schema::num_doc.eq(&self.num_doc),
                    schema::descr_aj.eq(&self.descr_aj),
                    schema::dt_ref.eq(&self.dt_ref),
                ))
                .execute(&mut conn)?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "EfdM620".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM620 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM620, [reg, ind_aj, vl_aj, cod_aj, num_doc, descr_aj, dt_ref]);
register_model!(EfdM620, "m620");
