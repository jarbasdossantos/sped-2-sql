use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_p100::dsl as schema;
use crate::schemas::efd_p100::table;
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
#[diesel(table_name = crate::schemas::efd_p100::dsl)]
pub struct EfdP100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fim: Option<String>,
    pub vl_rec_tot_est: Option<String>,
    pub cod_ativ_econ: Option<String>,
    pub vl_rec_ativ_estab: Option<String>,
    pub vl_exc: Option<String>,
    pub vl_bc_cont: Option<String>,
    pub aliq_cont: Option<String>,
    pub vl_cont_apu: Option<String>,
    pub cod_cta: Option<String>,
    pub info_compl: Option<String>,
}

#[async_trait]
impl Model for EfdP100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdP100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            dt_ini: get_field(&fields, 2),
            dt_fim: get_field(&fields, 3),
            vl_rec_tot_est: get_field(&fields, 4),
            cod_ativ_econ: get_field(&fields, 5),
            vl_rec_ativ_estab: get_field(&fields, 6),
            vl_exc: get_field(&fields, 7),
            vl_bc_cont: get_field(&fields, 8),
            aliq_cont: get_field(&fields, 9),
            vl_cont_apu: get_field(&fields, 10),
            cod_cta: get_field(&fields, 11),
            info_compl: get_field(&fields, 12),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdP100>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdP100::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdP100::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::dt_ini.eq(&self.dt_ini),
                    schema::dt_fim.eq(&self.dt_fim),
                    schema::vl_rec_tot_est.eq(&self.vl_rec_tot_est),
                    schema::cod_ativ_econ.eq(&self.cod_ativ_econ),
                    schema::vl_rec_ativ_estab.eq(&self.vl_rec_ativ_estab),
                    schema::vl_exc.eq(&self.vl_exc),
                    schema::vl_bc_cont.eq(&self.vl_bc_cont),
                    schema::aliq_cont.eq(&self.aliq_cont),
                    schema::vl_cont_apu.eq(&self.vl_cont_apu),
                    schema::cod_cta.eq(&self.cod_cta),
                    schema::info_compl.eq(&self.info_compl),
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
        "EfdP100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdP100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdP100, [reg, dt_ini, dt_fim, vl_rec_tot_est, cod_ativ_econ, vl_rec_ativ_estab, vl_exc, vl_bc_cont, aliq_cont, vl_cont_apu, cod_cta, info_compl]);
register_model!(EfdP100, "p100");
