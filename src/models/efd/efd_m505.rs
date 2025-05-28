use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m505::efd_m505::dsl as schema;
use crate::schemas::efd_m505::efd_m505::table;
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
#[diesel(table_name = crate::schemas::efd_m505::efd_m505::dsl)]
pub struct EfdM505 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub cst_cofins: Option<String>,
    pub vl_bc_cofins_tot: Option<String>,
    pub vl_bc_cofins_cum: Option<String>,
    pub vl_bc_cofins_nc: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub quant_bc_cofins_tot: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub desc_cred: Option<String>,
}

#[async_trait]
impl Model for EfdM505 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM505 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_bc_cred: get_field(&fields, 2),
            cst_cofins: get_field(&fields, 3),
            vl_bc_cofins_tot: get_field(&fields, 4),
            vl_bc_cofins_cum: get_field(&fields, 5),
            vl_bc_cofins_nc: get_field(&fields, 6),
            vl_bc_cofins: get_field(&fields, 7),
            quant_bc_cofins_tot: get_field(&fields, 8),
            quant_bc_cofins: get_field(&fields, 9),
            desc_cred: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM505>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM505::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM505::as_select())
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
                    schema::nat_bc_cred.eq(&self.nat_bc_cred),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::vl_bc_cofins_tot.eq(&self.vl_bc_cofins_tot),
                    schema::vl_bc_cofins_cum.eq(&self.vl_bc_cofins_cum),
                    schema::vl_bc_cofins_nc.eq(&self.vl_bc_cofins_nc),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::quant_bc_cofins_tot.eq(&self.quant_bc_cofins_tot),
                    schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
                    schema::desc_cred.eq(&self.desc_cred),
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
        "EfdM505".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM505 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    EfdM505,
    [
        reg,
        nat_bc_cred,
        cst_cofins,
        vl_bc_cofins_tot,
        vl_bc_cofins_cum,
        vl_bc_cofins_nc,
        vl_bc_cofins,
        quant_bc_cofins_tot,
        quant_bc_cofins,
        desc_cred
    ]
);
register_model!(EfdM505, "m505");
