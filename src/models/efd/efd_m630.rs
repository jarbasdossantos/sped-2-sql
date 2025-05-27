use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m630::efd_m630::dsl as schema;
use crate::schemas::efd_m630::efd_m630::table;
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
#[diesel(table_name = crate::schemas::efd_m630::efd_m630::dsl)]
pub struct EfdM630 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cnpj: Option<String>,
    pub vl_vend: Option<String>,
    pub vl_nao_receb: Option<String>,
    pub vl_cont_dif: Option<String>,
    pub vl_cred_dif: Option<String>,
    pub cod_cred: Option<String>,
}

#[async_trait]
impl Model for EfdM630 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM630 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cnpj: get_field(&fields, 2),
            vl_vend: get_field(&fields, 3),
            vl_nao_receb: get_field(&fields, 4),
            vl_cont_dif: get_field(&fields, 5),
            vl_cred_dif: get_field(&fields, 6),
            cod_cred: get_field(&fields, 7),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM630>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdM630::as_select())
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
                    schema::cnpj.eq(&self.cnpj),
                    schema::vl_vend.eq(&self.vl_vend),
                    schema::vl_nao_receb.eq(&self.vl_nao_receb),
                    schema::vl_cont_dif.eq(&self.vl_cont_dif),
                    schema::vl_cred_dif.eq(&self.vl_cred_dif),
                    schema::cod_cred.eq(&self.cod_cred),
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
        "EfdM630".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM630 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM630, [reg, cnpj, vl_vend, vl_nao_receb, vl_cont_dif, vl_cred_dif, cod_cred]);
register_model!(EfdM630, "m630");
