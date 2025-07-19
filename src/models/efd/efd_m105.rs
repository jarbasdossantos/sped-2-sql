use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m105::dsl as schema;
use crate::schemas::efd_m105::table;
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
use diesel::sqlite::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::efd_m105::dsl)]
pub struct EfdM105 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_bc_pis_tot: Option<String>,
    pub vl_bc_pis_cum: Option<String>,
    pub vl_bc_pis_nc: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub quant_bc_pis_tot: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub desc_cred: Option<String>,
}

#[async_trait]
impl Model for EfdM105 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM105 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_bc_cred: get_field(&fields, 2),
            cst_pis: get_field(&fields, 3),
            vl_bc_pis_tot: get_field(&fields, 4),
            vl_bc_pis_cum: get_field(&fields, 5),
            vl_bc_pis_nc: get_field(&fields, 6),
            vl_bc_pis: get_field(&fields, 7),
            quant_bc_pis_tot: get_field(&fields, 8),
            quant_bc_pis: get_field(&fields, 9),
            desc_cred: get_field(&fields, 10),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdM105>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM105::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM105::as_select())
                .load(conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = DB_POOL.lock().await.get().unwrap();

            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::nat_bc_cred.eq(&self.nat_bc_cred),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::vl_bc_pis_tot.eq(&self.vl_bc_pis_tot),
                    schema::vl_bc_pis_cum.eq(&self.vl_bc_pis_cum),
                    schema::vl_bc_pis_nc.eq(&self.vl_bc_pis_nc),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::quant_bc_pis_tot.eq(&self.quant_bc_pis_tot),
                    schema::quant_bc_pis.eq(&self.quant_bc_pis),
                    schema::desc_cred.eq(&self.desc_cred),
                ))
                .execute(&mut conn)?;

            Ok(sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?)
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "EfdM105".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM105 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM105, [reg, nat_bc_cred, cst_pis, vl_bc_pis_tot, vl_bc_pis_cum, vl_bc_pis_nc, vl_bc_pis, quant_bc_pis_tot, quant_bc_pis, desc_cred]);
register_model!(EfdM105, "m105");
