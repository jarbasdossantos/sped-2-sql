use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_1100::dsl as schema;
use crate::schemas::efd_1100::table;
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
#[diesel(table_name = crate::schemas::efd_1100::dsl)]
pub struct Efd1100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub per_apu_cred: Option<String>,
    pub orig_cred: Option<String>,
    pub cnpj_suc: Option<String>,
    pub cod_cred: Option<String>,
    pub vl_cred_apu: Option<String>,
    pub vl_cred_ext_apu: Option<String>,
    pub vl_tot_cred_apu: Option<String>,
    pub vl_cred_desc_pa_ant: Option<String>,
    pub vl_cred_per_pa_ant: Option<String>,
    pub vl_cred_dcomp_pa_ant: Option<String>,
    pub sd_cred_disp_efd: Option<String>,
    pub vl_cred_desc_efd: Option<String>,
    pub vl_cred_per_efd: Option<String>,
    pub vl_cred_dcomp_efd: Option<String>,
    pub vl_cred_trans: Option<String>,
    pub vl_cred_out: Option<String>,
    pub sld_cred_fim: Option<String>,
}

#[async_trait]
impl Model for Efd1100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd1100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            per_apu_cred: get_field(&fields, 2),
            orig_cred: get_field(&fields, 3),
            cnpj_suc: get_field(&fields, 4),
            cod_cred: get_field(&fields, 5),
            vl_cred_apu: get_field(&fields, 6),
            vl_cred_ext_apu: get_field(&fields, 7),
            vl_tot_cred_apu: get_field(&fields, 8),
            vl_cred_desc_pa_ant: get_field(&fields, 9),
            vl_cred_per_pa_ant: get_field(&fields, 10),
            vl_cred_dcomp_pa_ant: get_field(&fields, 11),
            sd_cred_disp_efd: get_field(&fields, 12),
            vl_cred_desc_efd: get_field(&fields, 13),
            vl_cred_per_efd: get_field(&fields, 14),
            vl_cred_dcomp_efd: get_field(&fields, 15),
            vl_cred_trans: get_field(&fields, 16),
            vl_cred_out: get_field(&fields, 17),
            sld_cred_fim: get_field(&fields, 18),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd1100>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd1100::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd1100::as_select())
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
                    schema::per_apu_cred.eq(&self.per_apu_cred),
                    schema::orig_cred.eq(&self.orig_cred),
                    schema::cnpj_suc.eq(&self.cnpj_suc),
                    schema::cod_cred.eq(&self.cod_cred),
                    schema::vl_cred_apu.eq(&self.vl_cred_apu),
                    schema::vl_cred_ext_apu.eq(&self.vl_cred_ext_apu),
                    schema::vl_tot_cred_apu.eq(&self.vl_tot_cred_apu),
                    schema::vl_cred_desc_pa_ant.eq(&self.vl_cred_desc_pa_ant),
                    schema::vl_cred_per_pa_ant.eq(&self.vl_cred_per_pa_ant),
                    schema::vl_cred_dcomp_pa_ant.eq(&self.vl_cred_dcomp_pa_ant),
                    schema::sd_cred_disp_efd.eq(&self.sd_cred_disp_efd),
                    schema::vl_cred_desc_efd.eq(&self.vl_cred_desc_efd),
                    schema::vl_cred_per_efd.eq(&self.vl_cred_per_efd),
                    schema::vl_cred_dcomp_efd.eq(&self.vl_cred_dcomp_efd),
                    schema::vl_cred_trans.eq(&self.vl_cred_trans),
                    schema::vl_cred_out.eq(&self.vl_cred_out),
                    schema::sld_cred_fim.eq(&self.sld_cred_fim),
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
        "Efd1100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd1100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd1100, [reg, per_apu_cred, orig_cred, cnpj_suc, cod_cred, vl_cred_apu, vl_cred_ext_apu, vl_tot_cred_apu, vl_cred_desc_pa_ant, vl_cred_per_pa_ant, vl_cred_dcomp_pa_ant, sd_cred_disp_efd, vl_cred_desc_efd, vl_cred_per_efd, vl_cred_dcomp_efd, vl_cred_trans, vl_cred_out, sld_cred_fim]);
register_model!(Efd1100, "1100");
