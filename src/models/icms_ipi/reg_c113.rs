use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c113::dsl as schema;
use crate::schemas::reg_c113::table;
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
#[diesel(table_name = crate::schemas::reg_c113::dsl)]
pub struct RegC113 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_oper: Option<String>,
    pub ind_emit: Option<String>,
    pub cod_part: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
}

#[async_trait]
impl Model for RegC113 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC113 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_oper: get_field(&fields, 2),
            ind_emit: get_field(&fields, 3),
            cod_part: get_field(&fields, 4),
            cod_mod: get_field(&fields, 5),
            ser: get_field(&fields, 6),
            sub: get_field(&fields, 7),
            num_doc: get_field(&fields, 8),
            dt_doc: get_field(&fields, 9),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegC113>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC113::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC113::as_select())
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
                    schema::ind_oper.eq(&self.ind_oper),
                    schema::ind_emit.eq(&self.ind_emit),
                    schema::cod_part.eq(&self.cod_part),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
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
        "RegC113".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC113 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC113, [reg, ind_oper, ind_emit, cod_part, cod_mod, ser, sub, num_doc, dt_doc]);
register_model!(RegC113, "c113");
