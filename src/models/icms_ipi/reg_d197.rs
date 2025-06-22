use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d197::dsl as schema;
use crate::schemas::reg_d197::table;
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
#[diesel(table_name = crate::schemas::reg_d197::dsl)]
pub struct RegD197 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_aj: Option<String>,
    pub descr_compl_aj: Option<String>,
    pub cod_item: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub aliq_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_outros: Option<String>,
}

#[async_trait]
impl Model for RegD197 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD197 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_aj: get_field(&fields, 2),
            descr_compl_aj: get_field(&fields, 3),
            cod_item: get_field(&fields, 4),
            vl_bc_icms: get_field(&fields, 5),
            aliq_icms: get_field(&fields, 6),
            vl_icms: get_field(&fields, 7),
            vl_outros: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD197>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD197::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD197::as_select())
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
                    schema::cod_aj.eq(&self.cod_aj),
                    schema::descr_compl_aj.eq(&self.descr_compl_aj),
                    schema::cod_item.eq(&self.cod_item),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::aliq_icms.eq(&self.aliq_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_outros.eq(&self.vl_outros),
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
        "RegD197".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD197 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD197, [reg, cod_aj, descr_compl_aj, cod_item, vl_bc_icms, aliq_icms, vl_icms, vl_outros]);
register_model!(RegD197, "d197");
