use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c385::efd_c385::dsl as schema;
use crate::schemas::efd_c385::efd_c385::table;
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
#[diesel(table_name = crate::schemas::efd_c385::efd_c385::dsl)]
pub struct EfdC385 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cst_cofins: Option<String>,
    pub cod_item: Option<String>,
    pub vl_item: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub aliq_cofins_quant: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdC385 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC385 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cst_cofins: get_field(&fields, 2),
            cod_item: get_field(&fields, 3),
            vl_item: get_field(&fields, 4),
            vl_bc_cofins: get_field(&fields, 5),
            aliq_cofins: get_field(&fields, 6),
            quant_bc_cofins: get_field(&fields, 7),
            aliq_cofins_quant: get_field(&fields, 8),
            vl_cofins: get_field(&fields, 9),
            cod_cta: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC385>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdC385::as_select())
            .load(&mut DB_POOL
                .get().unwrap())?)
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::cod_item.eq(&self.cod_item),
                    schema::vl_item.eq(&self.vl_item),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
                    schema::aliq_cofins_quant.eq(&self.aliq_cofins_quant),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::cod_cta.eq(&self.cod_cta),
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
        "EfdC385".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC385 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC385, [reg, cst_cofins, cod_item, vl_item, vl_bc_cofins, aliq_cofins, quant_bc_cofins, aliq_cofins_quant, vl_cofins, cod_cta]);
register_model!(EfdC385, "c385");
