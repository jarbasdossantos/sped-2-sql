use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m210::dsl as schema;
use crate::schemas::efd_m210::table;
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
#[diesel(table_name = crate::schemas::efd_m210::dsl)]
pub struct EfdM210 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_cont: Option<String>,
    pub vl_rec_brt: Option<String>,
    pub vl_bc_cont: Option<String>,
    pub vl_ajus_acres_bc_pis: Option<String>,
    pub vl_ajus_reduc_bc_pis: Option<String>,
    pub vl_bc_cont_ajus: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_cont_apur: Option<String>,
    pub vl_ajus_acres: Option<String>,
    pub vl_ajus_reduc: Option<String>,
    pub vl_cont_difer: Option<String>,
    pub vl_cont_difer_ant: Option<String>,
    pub vl_cont_per: Option<String>,
}

#[async_trait]
impl Model for EfdM210 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM210 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_cont: get_field(&fields, 2),
            vl_rec_brt: get_field(&fields, 3),
            vl_bc_cont: get_field(&fields, 4),
            vl_ajus_acres_bc_pis: get_field(&fields, 5),
            vl_ajus_reduc_bc_pis: get_field(&fields, 6),
            vl_bc_cont_ajus: get_field(&fields, 7),
            aliq_pis: get_field(&fields, 8),
            quant_bc_pis: get_field(&fields, 9),
            aliq_pis_quant: get_field(&fields, 10),
            vl_cont_apur: get_field(&fields, 11),
            vl_ajus_acres: get_field(&fields, 12),
            vl_ajus_reduc: get_field(&fields, 13),
            vl_cont_difer: get_field(&fields, 14),
            vl_cont_difer_ant: get_field(&fields, 15),
            vl_cont_per: get_field(&fields, 16),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM210>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM210::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM210::as_select())
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
                    schema::cod_cont.eq(&self.cod_cont),
                    schema::vl_rec_brt.eq(&self.vl_rec_brt),
                    schema::vl_bc_cont.eq(&self.vl_bc_cont),
                    schema::vl_ajus_acres_bc_pis.eq(&self.vl_ajus_acres_bc_pis),
                    schema::vl_ajus_reduc_bc_pis.eq(&self.vl_ajus_reduc_bc_pis),
                    schema::vl_bc_cont_ajus.eq(&self.vl_bc_cont_ajus),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::quant_bc_pis.eq(&self.quant_bc_pis),
                    schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
                    schema::vl_cont_apur.eq(&self.vl_cont_apur),
                    schema::vl_ajus_acres.eq(&self.vl_ajus_acres),
                    schema::vl_ajus_reduc.eq(&self.vl_ajus_reduc),
                    schema::vl_cont_difer.eq(&self.vl_cont_difer),
                    schema::vl_cont_difer_ant.eq(&self.vl_cont_difer_ant),
                    schema::vl_cont_per.eq(&self.vl_cont_per),
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
        "EfdM210".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM210 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM210, [reg, cod_cont, vl_rec_brt, vl_bc_cont, vl_ajus_acres_bc_pis, vl_ajus_reduc_bc_pis, vl_bc_cont_ajus, aliq_pis, quant_bc_pis, aliq_pis_quant, vl_cont_apur, vl_ajus_acres, vl_ajus_reduc, vl_cont_difer, vl_cont_difer_ant, vl_cont_per]);
register_model!(EfdM210, "m210");
