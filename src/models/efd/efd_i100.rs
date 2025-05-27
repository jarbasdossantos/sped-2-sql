use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_i100::efd_i100::dsl as schema;
use crate::schemas::efd_i100::efd_i100::table;
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
#[diesel(table_name = crate::schemas::efd_i100::efd_i100::dsl)]
pub struct EfdI100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_rec_fin: Option<String>,
    pub cst: Option<String>,
    pub vl_tot_ded_ger: Option<String>,
    pub vl_tot_ded_esp: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cofins: Option<String>,
    pub inf_comp: Option<String>,
}

#[async_trait]
impl Model for EfdI100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdI100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_rec_fin: get_field(&fields, 2),
            cst: get_field(&fields, 3),
            vl_tot_ded_ger: get_field(&fields, 4),
            vl_tot_ded_esp: get_field(&fields, 5),
            vl_bc_pis: get_field(&fields, 6),
            aliq_pis: get_field(&fields, 7),
            vl_pis: get_field(&fields, 8),
            vl_bc_cofins: get_field(&fields, 9),
            aliq_cofins: get_field(&fields, 10),
            vl_cofins: get_field(&fields, 11),
            inf_comp: get_field(&fields, 12),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdI100>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdI100::as_select())
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
                    schema::vl_rec_fin.eq(&self.vl_rec_fin),
                    schema::cst.eq(&self.cst),
                    schema::vl_tot_ded_ger.eq(&self.vl_tot_ded_ger),
                    schema::vl_tot_ded_esp.eq(&self.vl_tot_ded_esp),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::inf_comp.eq(&self.inf_comp),
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
        "EfdI100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdI100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdI100, [reg, vl_rec_fin, cst, vl_tot_ded_ger, vl_tot_ded_esp, vl_bc_pis, aliq_pis, vl_pis, vl_bc_cofins, aliq_cofins, vl_cofins, inf_comp]);
register_model!(EfdI100, "i100");
