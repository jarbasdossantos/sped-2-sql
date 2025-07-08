use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m600::dsl as schema;
use crate::schemas::efd_m600::table;
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
#[diesel(table_name = crate::schemas::efd_m600::dsl)]
pub struct EfdM600 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_tot_cont_nc_per: Option<String>,
    pub vl_tot_cred_desc: Option<String>,
    pub vl_tot_cred_desc_ant: Option<String>,
    pub vl_tot_cont_nc_dev: Option<String>,
    pub vl_ret_nc: Option<String>,
    pub vl_out_ded_nc: Option<String>,
    pub vl_cont_nc_rec: Option<String>,
    pub vl_tot_cont_cum_per: Option<String>,
    pub vl_ret_cum: Option<String>,
    pub vl_out_ded_cum: Option<String>,
    pub vl_cont_cum_rec: Option<String>,
    pub vl_tot_cont_rec: Option<String>,
}

#[async_trait]
impl Model for EfdM600 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM600 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_tot_cont_nc_per: get_field(&fields, 2),
            vl_tot_cred_desc: get_field(&fields, 3),
            vl_tot_cred_desc_ant: get_field(&fields, 4),
            vl_tot_cont_nc_dev: get_field(&fields, 5),
            vl_ret_nc: get_field(&fields, 6),
            vl_out_ded_nc: get_field(&fields, 7),
            vl_cont_nc_rec: get_field(&fields, 8),
            vl_tot_cont_cum_per: get_field(&fields, 9),
            vl_ret_cum: get_field(&fields, 10),
            vl_out_ded_cum: get_field(&fields, 11),
            vl_cont_cum_rec: get_field(&fields, 12),
            vl_tot_cont_rec: get_field(&fields, 13),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM600>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM600::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM600::as_select())
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
                    schema::vl_tot_cont_nc_per.eq(&self.vl_tot_cont_nc_per),
                    schema::vl_tot_cred_desc.eq(&self.vl_tot_cred_desc),
                    schema::vl_tot_cred_desc_ant.eq(&self.vl_tot_cred_desc_ant),
                    schema::vl_tot_cont_nc_dev.eq(&self.vl_tot_cont_nc_dev),
                    schema::vl_ret_nc.eq(&self.vl_ret_nc),
                    schema::vl_out_ded_nc.eq(&self.vl_out_ded_nc),
                    schema::vl_cont_nc_rec.eq(&self.vl_cont_nc_rec),
                    schema::vl_tot_cont_cum_per.eq(&self.vl_tot_cont_cum_per),
                    schema::vl_ret_cum.eq(&self.vl_ret_cum),
                    schema::vl_out_ded_cum.eq(&self.vl_out_ded_cum),
                    schema::vl_cont_cum_rec.eq(&self.vl_cont_cum_rec),
                    schema::vl_tot_cont_rec.eq(&self.vl_tot_cont_rec),
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
        "EfdM600".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM600 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM600, [reg, vl_tot_cont_nc_per, vl_tot_cred_desc, vl_tot_cred_desc_ant, vl_tot_cont_nc_dev, vl_ret_nc, vl_out_ded_nc, vl_cont_nc_rec, vl_tot_cont_cum_per, vl_ret_cum, vl_out_ded_cum, vl_cont_cum_rec, vl_tot_cont_rec]);
register_model!(EfdM600, "m600");
