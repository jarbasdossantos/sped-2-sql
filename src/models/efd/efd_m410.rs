use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m410::dsl as schema;
use crate::schemas::efd_m410::table;
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
use diesel::sqlite::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::efd_m410::dsl)]
pub struct EfdM410 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_rec: Option<String>,
    pub vl_rec: Option<String>,
    pub cod_cta: Option<String>,
    pub desc_compl: Option<String>,
}

#[async_trait]
impl Model for EfdM410 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM410 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_rec: get_field(&fields, 2),
            vl_rec: get_field(&fields, 3),
            cod_cta: get_field(&fields, 4),
            desc_compl: get_field(&fields, 5),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdM410>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM410::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM410::as_select())
                .load(conn)?)
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
                    schema::nat_rec.eq(&self.nat_rec),
                    schema::vl_rec.eq(&self.vl_rec),
                    schema::cod_cta.eq(&self.cod_cta),
                    schema::desc_compl.eq(&self.desc_compl),
                ))
                .execute(&mut conn)?;

            Ok(sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?)
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "EfdM410".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM410 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM410, [reg, nat_rec, vl_rec, cod_cta, desc_compl]);
register_model!(EfdM410, "m410");
