use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_k255::dsl as schema;
use crate::schemas::reg_k255::table;
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
#[diesel(table_name = crate::schemas::reg_k255::dsl)]
pub struct RegK255 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_cons: Option<String>,
    pub cod_item: Option<String>,
    pub qtd: Option<String>,
    pub cod_ins_subst: Option<String>,
}

#[async_trait]
impl Model for RegK255 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegK255 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            dt_cons: get_field(&fields, 2),
            cod_item: get_field(&fields, 3),
            qtd: get_field(&fields, 4),
            cod_ins_subst: get_field(&fields, 5),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegK255>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegK255::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegK255::as_select())
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
                    schema::dt_cons.eq(&self.dt_cons),
                    schema::cod_item.eq(&self.cod_item),
                    schema::qtd.eq(&self.qtd),
                    schema::cod_ins_subst.eq(&self.cod_ins_subst),
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
        "RegK255".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegK255 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegK255, [reg, dt_cons, cod_item, qtd, cod_ins_subst]);
register_model!(RegK255, "k255");
