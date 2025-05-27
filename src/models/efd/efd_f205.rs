use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f205::efd_f205::dsl as schema;
use crate::schemas::efd_f205::efd_f205::table;
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
#[diesel(table_name = crate::schemas::efd_f205::efd_f205::dsl)]
pub struct EfdF205 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_cus_inc_acum_ant: Option<String>,
    pub vl_cus_inc_per_esc: Option<String>,
    pub vl_cus_inc_acum: Option<String>,
    pub vl_exc_bc_cus_inc_acum: Option<String>,
    pub vl_bc_cus_inc: Option<String>,
    pub cst_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_cred_pis_acum: Option<String>,
    pub vl_cred_pis_desc_ant: Option<String>,
    pub vl_cred_pis_desc: Option<String>,
    pub vl_cred_pis_desc_fut: Option<String>,
    pub cst_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cred_cofins_acum: Option<String>,
    pub vl_cred_cofins_desc_ant: Option<String>,
    pub vl_cred_cofins_desc: Option<String>,
    pub vl_cred_cofins_desc_fut: Option<String>,
}

#[async_trait]
impl Model for EfdF205 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF205 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    vl_cus_inc_acum_ant: get_field(&fields, 2),
        vl_cus_inc_per_esc: get_field(&fields, 3),
        vl_cus_inc_acum: get_field(&fields, 4),
        vl_exc_bc_cus_inc_acum: get_field(&fields, 5),
        vl_bc_cus_inc: get_field(&fields, 6),
        cst_pis: get_field(&fields, 7),
        aliq_pis: get_field(&fields, 8),
        vl_cred_pis_acum: get_field(&fields, 9),
        vl_cred_pis_desc_ant: get_field(&fields, 10),
        vl_cred_pis_desc: get_field(&fields, 11),
        vl_cred_pis_desc_fut: get_field(&fields, 12),
        cst_cofins: get_field(&fields, 13),
        aliq_cofins: get_field(&fields, 14),
        vl_cred_cofins_acum: get_field(&fields, 15),
        vl_cred_cofins_desc_ant: get_field(&fields, 16),
        vl_cred_cofins_desc: get_field(&fields, 17),
        vl_cred_cofins_desc_fut: get_field(&fields, 18),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF205>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdF205::as_select())
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
            schema::vl_cus_inc_acum_ant.eq(&self.vl_cus_inc_acum_ant),
schema::vl_cus_inc_per_esc.eq(&self.vl_cus_inc_per_esc),
schema::vl_cus_inc_acum.eq(&self.vl_cus_inc_acum),
schema::vl_exc_bc_cus_inc_acum.eq(&self.vl_exc_bc_cus_inc_acum),
schema::vl_bc_cus_inc.eq(&self.vl_bc_cus_inc),
schema::cst_pis.eq(&self.cst_pis),
schema::aliq_pis.eq(&self.aliq_pis),
schema::vl_cred_pis_acum.eq(&self.vl_cred_pis_acum),
schema::vl_cred_pis_desc_ant.eq(&self.vl_cred_pis_desc_ant),
schema::vl_cred_pis_desc.eq(&self.vl_cred_pis_desc),
schema::vl_cred_pis_desc_fut.eq(&self.vl_cred_pis_desc_fut),
schema::cst_cofins.eq(&self.cst_cofins),
schema::aliq_cofins.eq(&self.aliq_cofins),
schema::vl_cred_cofins_acum.eq(&self.vl_cred_cofins_acum),
schema::vl_cred_cofins_desc_ant.eq(&self.vl_cred_cofins_desc_ant),
schema::vl_cred_cofins_desc.eq(&self.vl_cred_cofins_desc),
schema::vl_cred_cofins_desc_fut.eq(&self.vl_cred_cofins_desc_fut),
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
        "EfdF205".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF205 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF205, [reg, vl_cus_inc_acum_ant, vl_cus_inc_per_esc, vl_cus_inc_acum, vl_exc_bc_cus_inc_acum, vl_bc_cus_inc, cst_pis, aliq_pis, vl_cred_pis_acum, vl_cred_pis_desc_ant, vl_cred_pis_desc, vl_cred_pis_desc_fut, cst_cofins, aliq_cofins, vl_cred_cofins_acum, vl_cred_cofins_desc_ant, vl_cred_cofins_desc, vl_cred_cofins_desc_fut]);
register_model!(EfdF205, "f205");
