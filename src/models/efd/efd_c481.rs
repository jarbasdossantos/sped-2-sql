use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c481::efd_c481::dsl as schema;
use crate::schemas::efd_c481::efd_c481::table;
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
#[diesel(table_name = crate::schemas::efd_c481::efd_c481::dsl)]
pub struct EfdC481 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_item: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cod_item: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdC481 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC481 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    cst_pis: get_field(&fields, 2),
        vl_item: get_field(&fields, 3),
        vl_bc_pis: get_field(&fields, 4),
        aliq_pis: get_field(&fields, 5),
        quant_bc_pis: get_field(&fields, 6),
        aliq_pis_quant: get_field(&fields, 7),
        vl_pis: get_field(&fields, 8),
        cod_item: get_field(&fields, 9),
        cod_cta: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC481>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdC481::as_select())
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
            schema::cst_pis.eq(&self.cst_pis),
schema::vl_item.eq(&self.vl_item),
schema::vl_bc_pis.eq(&self.vl_bc_pis),
schema::aliq_pis.eq(&self.aliq_pis),
schema::quant_bc_pis.eq(&self.quant_bc_pis),
schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
schema::vl_pis.eq(&self.vl_pis),
schema::cod_item.eq(&self.cod_item),
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
        "EfdC481".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC481 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC481, [reg, cst_pis, vl_item, vl_bc_pis, aliq_pis, quant_bc_pis, aliq_pis_quant, vl_pis, cod_item, cod_cta]);
register_model!(EfdC481, "c481");
