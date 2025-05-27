use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c491::efd_c491::dsl as schema;
use crate::schemas::efd_c491::efd_c491::table;
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
#[diesel(table_name = crate::schemas::efd_c491::efd_c491::dsl)]
pub struct EfdC491 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_item: Option<String>,
    pub cst_pis: Option<String>,
    pub cfop: Option<String>,
    pub vl_item: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdC491 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC491 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_item: get_field(&fields, 2),
            cst_pis: get_field(&fields, 3),
            cfop: get_field(&fields, 4),
            vl_item: get_field(&fields, 5),
            vl_bc_pis: get_field(&fields, 6),
            aliq_pis: get_field(&fields, 7),
            quant_bc_pis: get_field(&fields, 8),
            aliq_pis_quant: get_field(&fields, 9),
            vl_pis: get_field(&fields, 10),
            cod_cta: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC491>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdC491::as_select())
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
                    schema::cod_item.eq(&self.cod_item),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::cfop.eq(&self.cfop),
                    schema::vl_item.eq(&self.vl_item),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::quant_bc_pis.eq(&self.quant_bc_pis),
                    schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
                    schema::vl_pis.eq(&self.vl_pis),
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
        "EfdC491".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC491 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC491, [reg, cod_item, cst_pis, cfop, vl_item, vl_bc_pis, aliq_pis, quant_bc_pis, aliq_pis_quant, vl_pis, cod_cta]);
register_model!(EfdC491, "c491");
