use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m300::efd_m300::dsl as schema;
use crate::schemas::efd_m300::efd_m300::table;
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
#[diesel(table_name = crate::schemas::efd_m300::efd_m300::dsl)]
pub struct EfdM300 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_cont: Option<String>,
    pub vl_cont_apur_difer: Option<String>,
    pub nat_cred_desc: Option<String>,
    pub vl_cred_desc_difer: Option<String>,
    pub vl_cont_difer_ant: Option<String>,
    pub per_apur: Option<String>,
    pub dt_receb: Option<String>,
}

#[async_trait]
impl Model for EfdM300 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM300 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    cod_cont: get_field(&fields, 2),
        vl_cont_apur_difer: get_field(&fields, 3),
        nat_cred_desc: get_field(&fields, 4),
        vl_cred_desc_difer: get_field(&fields, 5),
        vl_cont_difer_ant: get_field(&fields, 6),
        per_apur: get_field(&fields, 7),
        dt_receb: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM300>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdM300::as_select())
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
            schema::cod_cont.eq(&self.cod_cont),
schema::vl_cont_apur_difer.eq(&self.vl_cont_apur_difer),
schema::nat_cred_desc.eq(&self.nat_cred_desc),
schema::vl_cred_desc_difer.eq(&self.vl_cred_desc_difer),
schema::vl_cont_difer_ant.eq(&self.vl_cont_difer_ant),
schema::per_apur.eq(&self.per_apur),
schema::dt_receb.eq(&self.dt_receb),
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
        "EfdM300".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM300 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM300, [reg, cod_cont, vl_cont_apur_difer, nat_cred_desc, vl_cred_desc_difer, vl_cont_difer_ant, per_apur, dt_receb]);
register_model!(EfdM300, "m300");
