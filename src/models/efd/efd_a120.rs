use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_a120::dsl as schema;
use crate::schemas::efd_a120::table;
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
#[diesel(table_name = crate::schemas::efd_a120::dsl)]
pub struct EfdA120 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_tot_serv: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub vl_pis_imp: Option<String>,
    pub dt_pag_pis: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub vl_cofins_imp: Option<String>,
    pub dt_pag_cofins: Option<String>,
    pub loc_exe_serv: Option<String>,
}

#[async_trait]
impl Model for EfdA120 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdA120 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_tot_serv: get_field(&fields, 2),
            vl_bc_pis: get_field(&fields, 3),
            vl_pis_imp: get_field(&fields, 4),
            dt_pag_pis: get_field(&fields, 5),
            vl_bc_cofins: get_field(&fields, 6),
            vl_cofins_imp: get_field(&fields, 7),
            dt_pag_cofins: get_field(&fields, 8),
            loc_exe_serv: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdA120>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdA120::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdA120::as_select())
                .load(&mut conn)?)
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
                    schema::vl_tot_serv.eq(&self.vl_tot_serv),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::vl_pis_imp.eq(&self.vl_pis_imp),
                    schema::dt_pag_pis.eq(&self.dt_pag_pis),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::vl_cofins_imp.eq(&self.vl_cofins_imp),
                    schema::dt_pag_cofins.eq(&self.dt_pag_cofins),
                    schema::loc_exe_serv.eq(&self.loc_exe_serv),
                ))
                .execute(&mut conn)?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "EfdA120".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdA120 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdA120, [reg, vl_tot_serv, vl_bc_pis, vl_pis_imp, dt_pag_pis, vl_bc_cofins, vl_cofins_imp, dt_pag_cofins, loc_exe_serv]);
register_model!(EfdA120, "a120");
