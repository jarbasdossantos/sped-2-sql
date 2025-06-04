#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c500::efd_c500::dsl as schema;
use crate::schemas::efd_c500::efd_c500::table;
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
#[diesel(table_name = crate::schemas::efd_c500::efd_c500::dsl)]
pub struct EfdC500 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_part: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_sit: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub dt_e_s: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_icms: Option<String>,
    pub cod_inf: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
}

#[async_trait]
impl Model for EfdC500 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC500 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_part: get_field(&fields, 2),
            cod_mod: get_field(&fields, 3),
            cod_sit: get_field(&fields, 4),
            ser: get_field(&fields, 5),
            sub: get_field(&fields, 6),
            num_doc: get_field(&fields, 7),
            dt_doc: get_field(&fields, 8),
            dt_e_s: get_field(&fields, 9),
            vl_doc: get_field(&fields, 10),
            vl_icms: get_field(&fields, 11),
            cod_inf: get_field(&fields, 12),
            vl_pis: get_field(&fields, 13),
            vl_cofins: get_field(&fields, 14),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC500>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdC500::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdC500::as_select())
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
                    schema::cod_part.eq(&self.cod_part),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::dt_e_s.eq(&self.dt_e_s),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::cod_inf.eq(&self.cod_inf),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
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
        "EfdC500".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC500 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    EfdC500,
    [
        reg, cod_part, cod_mod, cod_sit, ser, sub, num_doc, dt_doc, dt_e_s, vl_doc, vl_icms,
        cod_inf, vl_pis, vl_cofins
    ]
);
register_model!(EfdC500, "c500");
