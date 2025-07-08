use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_1800::dsl as schema;
use crate::schemas::efd_1800::table;
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
#[diesel(table_name = crate::schemas::efd_1800::dsl)]
pub struct Efd1800 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub inc_imob: Option<String>,
    pub rec_receb_ret: Option<String>,
    pub rec_fin_ret: Option<String>,
    pub bc_ret: Option<String>,
    pub aliq_ret: Option<String>,
    pub vl_rec_uni: Option<String>,
    pub dt_rec_uni: Option<String>,
    pub cod_rec: Option<String>,
}

#[async_trait]
impl Model for Efd1800 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd1800 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            inc_imob: get_field(&fields, 2),
            rec_receb_ret: get_field(&fields, 3),
            rec_fin_ret: get_field(&fields, 4),
            bc_ret: get_field(&fields, 5),
            aliq_ret: get_field(&fields, 6),
            vl_rec_uni: get_field(&fields, 7),
            dt_rec_uni: get_field(&fields, 8),
            cod_rec: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd1800>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd1800::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd1800::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::inc_imob.eq(&self.inc_imob),
                    schema::rec_receb_ret.eq(&self.rec_receb_ret),
                    schema::rec_fin_ret.eq(&self.rec_fin_ret),
                    schema::bc_ret.eq(&self.bc_ret),
                    schema::aliq_ret.eq(&self.aliq_ret),
                    schema::vl_rec_uni.eq(&self.vl_rec_uni),
                    schema::dt_rec_uni.eq(&self.dt_rec_uni),
                    schema::cod_rec.eq(&self.cod_rec),
                ))
                .execute(&mut DB_POOL.lock().await.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.lock().await.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Efd1800".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd1800 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd1800, [reg, inc_imob, rec_receb_ret, rec_fin_ret, bc_ret, aliq_ret, vl_rec_uni, dt_rec_uni, cod_rec]);
register_model!(Efd1800, "1800");
