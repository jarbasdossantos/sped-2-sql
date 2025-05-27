use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f600::efd_f600::dsl as schema;
use crate::schemas::efd_f600::efd_f600::table;
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
#[diesel(table_name = crate::schemas::efd_f600::efd_f600::dsl)]
pub struct EfdF600 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_nat_ret: Option<String>,
    pub dt_ret: Option<String>,
    pub vl_bc_ret: Option<String>,
    pub vl_ret: Option<String>,
    pub cod_rec: Option<String>,
    pub ind_nat_rec: Option<String>,
    pub cnpj: Option<String>,
    pub vl_ret_pis: Option<String>,
    pub vl_ret_cofins: Option<String>,
    pub ind_dec: Option<String>,
}

#[async_trait]
impl Model for EfdF600 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF600 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    ind_nat_ret: get_field(&fields, 2),
        dt_ret: get_field(&fields, 3),
        vl_bc_ret: get_field(&fields, 4),
        vl_ret: get_field(&fields, 5),
        cod_rec: get_field(&fields, 6),
        ind_nat_rec: get_field(&fields, 7),
        cnpj: get_field(&fields, 8),
        vl_ret_pis: get_field(&fields, 9),
        vl_ret_cofins: get_field(&fields, 10),
        ind_dec: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF600>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdF600::as_select())
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
            schema::ind_nat_ret.eq(&self.ind_nat_ret),
schema::dt_ret.eq(&self.dt_ret),
schema::vl_bc_ret.eq(&self.vl_bc_ret),
schema::vl_ret.eq(&self.vl_ret),
schema::cod_rec.eq(&self.cod_rec),
schema::ind_nat_rec.eq(&self.ind_nat_rec),
schema::cnpj.eq(&self.cnpj),
schema::vl_ret_pis.eq(&self.vl_ret_pis),
schema::vl_ret_cofins.eq(&self.vl_ret_cofins),
schema::ind_dec.eq(&self.ind_dec),
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
        "EfdF600".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF600 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF600, [reg, ind_nat_ret, dt_ret, vl_bc_ret, vl_ret, cod_rec, ind_nat_rec, cnpj, vl_ret_pis, vl_ret_cofins, ind_dec]);
register_model!(EfdF600, "f600");
