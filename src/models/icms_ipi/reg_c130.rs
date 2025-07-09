use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c130::dsl as schema;
use crate::schemas::reg_c130::table;
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
#[diesel(table_name = crate::schemas::reg_c130::dsl)]
pub struct RegC130 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_serv_nt: Option<String>,
    pub vl_bc_issqn: Option<String>,
    pub vl_issqn: Option<String>,
    pub vl_bc_irrf: Option<String>,
    pub vl_irrf: Option<String>,
    pub vl_bc_prev: Option<String>,
    pub vl_prev: Option<String>,
}

#[async_trait]
impl Model for RegC130 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC130 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_serv_nt: get_field(&fields, 2),
            vl_bc_issqn: get_field(&fields, 3),
            vl_issqn: get_field(&fields, 4),
            vl_bc_irrf: get_field(&fields, 5),
            vl_irrf: get_field(&fields, 6),
            vl_bc_prev: get_field(&fields, 7),
            vl_prev: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC130>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC130::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC130::as_select())
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
                    schema::vl_serv_nt.eq(&self.vl_serv_nt),
                    schema::vl_bc_issqn.eq(&self.vl_bc_issqn),
                    schema::vl_issqn.eq(&self.vl_issqn),
                    schema::vl_bc_irrf.eq(&self.vl_bc_irrf),
                    schema::vl_irrf.eq(&self.vl_irrf),
                    schema::vl_bc_prev.eq(&self.vl_bc_prev),
                    schema::vl_prev.eq(&self.vl_prev),
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
        "RegC130".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC130 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC130, [reg, vl_serv_nt, vl_bc_issqn, vl_issqn, vl_bc_irrf, vl_irrf, vl_bc_prev, vl_prev]);
register_model!(RegC130, "c130");
