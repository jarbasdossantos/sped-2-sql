#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_k230::reg_k230::dsl as schema;
use crate::schemas::reg_k230::reg_k230::table;
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
#[diesel(table_name = crate::schemas::reg_k230::reg_k230::dsl)]
pub struct RegK230 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_ini_op: Option<String>,
    pub dt_fin_op: Option<String>,
    pub cod_doc_op: Option<String>,
    pub cod_item: Option<String>,
    pub qtd_enc: Option<String>,
}

#[async_trait]
impl Model for RegK230 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegK230 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            dt_ini_op: get_field(&fields, 2),
            dt_fin_op: get_field(&fields, 3),
            cod_doc_op: get_field(&fields, 4),
            cod_item: get_field(&fields, 5),
            qtd_enc: get_field(&fields, 6),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegK230>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegK230::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegK230::as_select())
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
                    schema::dt_ini_op.eq(&self.dt_ini_op),
                    schema::dt_fin_op.eq(&self.dt_fin_op),
                    schema::cod_doc_op.eq(&self.cod_doc_op),
                    schema::cod_item.eq(&self.cod_item),
                    schema::qtd_enc.eq(&self.qtd_enc),
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
        "RegK230".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegK230 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegK230,
    [reg, dt_ini_op, dt_fin_op, cod_doc_op, cod_item, qtd_enc]
);
register_model!(RegK230, "k230");
