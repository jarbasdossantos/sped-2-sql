use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1700::dsl as schema;
use crate::schemas::reg_1700::table;
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
#[diesel(table_name = crate::schemas::reg_1700::dsl)]
pub struct Reg1700 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_disp: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc_ini: Option<String>,
    pub num_doc_fin: Option<String>,
    pub num_aut: Option<String>,
}

#[async_trait]
impl Model for Reg1700 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1700 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_disp: get_field(&fields, 2),
            cod_mod: get_field(&fields, 3),
            ser: get_field(&fields, 4),
            sub: get_field(&fields, 5),
            num_doc_ini: get_field(&fields, 6),
            num_doc_fin: get_field(&fields, 7),
            num_aut: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1700>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1700::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1700::as_select())
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
                    schema::cod_disp.eq(&self.cod_disp),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc_ini.eq(&self.num_doc_ini),
                    schema::num_doc_fin.eq(&self.num_doc_fin),
                    schema::num_aut.eq(&self.num_aut),
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
        "Reg1700".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1700 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1700, [reg, cod_disp, cod_mod, ser, sub, num_doc_ini, num_doc_fin, num_aut]);
register_model!(Reg1700, "1700");
