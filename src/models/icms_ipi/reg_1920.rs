use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1920::dsl as schema;
use crate::schemas::reg_1920::table;
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
#[diesel(table_name = crate::schemas::reg_1920::dsl)]
pub struct Reg1920 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_tot_transf_debitos_oa: Option<String>,
    pub vl_tot_aj_debitos_oa: Option<String>,
    pub vl_estornos_cred_oa: Option<String>,
    pub vl_tot_transf_creditos_oa: Option<String>,
    pub vl_tot_aj_creditos_oa: Option<String>,
    pub vl_estornos_deb_oa: Option<String>,
    pub vl_sld_credor_ant_oa: Option<String>,
    pub vl_sld_apurado_oa: Option<String>,
    pub vl_tot_ded: Option<String>,
    pub vl_icms_recolher_oa: Option<String>,
    pub vl_sld_credor_transp_oa: Option<String>,
    pub deb_esp_oa: Option<String>,
}

#[async_trait]
impl Model for Reg1920 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1920 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_tot_transf_debitos_oa: get_field(&fields, 2),
            vl_tot_aj_debitos_oa: get_field(&fields, 3),
            vl_estornos_cred_oa: get_field(&fields, 4),
            vl_tot_transf_creditos_oa: get_field(&fields, 5),
            vl_tot_aj_creditos_oa: get_field(&fields, 6),
            vl_estornos_deb_oa: get_field(&fields, 7),
            vl_sld_credor_ant_oa: get_field(&fields, 8),
            vl_sld_apurado_oa: get_field(&fields, 9),
            vl_tot_ded: get_field(&fields, 10),
            vl_icms_recolher_oa: get_field(&fields, 11),
            vl_sld_credor_transp_oa: get_field(&fields, 12),
            deb_esp_oa: get_field(&fields, 13),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1920>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1920::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1920::as_select())
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
                    schema::vl_tot_transf_debitos_oa.eq(&self.vl_tot_transf_debitos_oa),
                    schema::vl_tot_aj_debitos_oa.eq(&self.vl_tot_aj_debitos_oa),
                    schema::vl_estornos_cred_oa.eq(&self.vl_estornos_cred_oa),
                    schema::vl_tot_transf_creditos_oa.eq(&self.vl_tot_transf_creditos_oa),
                    schema::vl_tot_aj_creditos_oa.eq(&self.vl_tot_aj_creditos_oa),
                    schema::vl_estornos_deb_oa.eq(&self.vl_estornos_deb_oa),
                    schema::vl_sld_credor_ant_oa.eq(&self.vl_sld_credor_ant_oa),
                    schema::vl_sld_apurado_oa.eq(&self.vl_sld_apurado_oa),
                    schema::vl_tot_ded.eq(&self.vl_tot_ded),
                    schema::vl_icms_recolher_oa.eq(&self.vl_icms_recolher_oa),
                    schema::vl_sld_credor_transp_oa.eq(&self.vl_sld_credor_transp_oa),
                    schema::deb_esp_oa.eq(&self.deb_esp_oa),
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
        "Reg1920".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1920 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1920, [reg, vl_tot_transf_debitos_oa, vl_tot_aj_debitos_oa, vl_estornos_cred_oa, vl_tot_transf_creditos_oa, vl_tot_aj_creditos_oa, vl_estornos_deb_oa, vl_sld_credor_ant_oa, vl_sld_apurado_oa, vl_tot_ded, vl_icms_recolher_oa, vl_sld_credor_transp_oa, deb_esp_oa]);
register_model!(Reg1920, "1920");
