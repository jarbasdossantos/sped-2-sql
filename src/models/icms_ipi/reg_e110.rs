use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_e110::reg_e110::dsl as schema;
use crate::schemas::reg_e110::reg_e110::table;
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
#[diesel(table_name = crate::schemas::reg_e110::reg_e110::dsl)]
pub struct RegE110 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_tot_debitos: Option<String>,
    pub vl_aj_debitos: Option<String>,
    pub vl_tot_aj_debitos: Option<String>,
    pub vl_estornos_cred: Option<String>,
    pub vl_tot_creditos: Option<String>,
    pub vl_aj_creditos: Option<String>,
    pub vl_tot_aj_creditos: Option<String>,
    pub vl_estornos_deb: Option<String>,
    pub vl_sld_credor_ant: Option<String>,
    pub vl_sld_apurado: Option<String>,
    pub vl_tot_ded: Option<String>,
    pub vl_icms_recolher: Option<String>,
    pub vl_sld_credor_transportar: Option<String>,
    pub deb_esp: Option<String>,
}

#[async_trait]
impl Model for RegE110 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegE110 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_tot_debitos: get_field(&fields, 2),
            vl_aj_debitos: get_field(&fields, 3),
            vl_tot_aj_debitos: get_field(&fields, 4),
            vl_estornos_cred: get_field(&fields, 5),
            vl_tot_creditos: get_field(&fields, 6),
            vl_aj_creditos: get_field(&fields, 7),
            vl_tot_aj_creditos: get_field(&fields, 8),
            vl_estornos_deb: get_field(&fields, 9),
            vl_sld_credor_ant: get_field(&fields, 10),
            vl_sld_apurado: get_field(&fields, 11),
            vl_tot_ded: get_field(&fields, 12),
            vl_icms_recolher: get_field(&fields, 13),
            vl_sld_credor_transportar: get_field(&fields, 14),
            deb_esp: get_field(&fields, 15),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegE110>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegE110::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegE110::as_select())
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
                    schema::vl_tot_debitos.eq(&self.vl_tot_debitos),
                    schema::vl_aj_debitos.eq(&self.vl_aj_debitos),
                    schema::vl_tot_aj_debitos.eq(&self.vl_tot_aj_debitos),
                    schema::vl_estornos_cred.eq(&self.vl_estornos_cred),
                    schema::vl_tot_creditos.eq(&self.vl_tot_creditos),
                    schema::vl_aj_creditos.eq(&self.vl_aj_creditos),
                    schema::vl_tot_aj_creditos.eq(&self.vl_tot_aj_creditos),
                    schema::vl_estornos_deb.eq(&self.vl_estornos_deb),
                    schema::vl_sld_credor_ant.eq(&self.vl_sld_credor_ant),
                    schema::vl_sld_apurado.eq(&self.vl_sld_apurado),
                    schema::vl_tot_ded.eq(&self.vl_tot_ded),
                    schema::vl_icms_recolher.eq(&self.vl_icms_recolher),
                    schema::vl_sld_credor_transportar.eq(&self.vl_sld_credor_transportar),
                    schema::deb_esp.eq(&self.deb_esp),
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
        "RegE110".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegE110 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegE110,
    [
        reg,
        vl_tot_debitos,
        vl_aj_debitos,
        vl_tot_aj_debitos,
        vl_estornos_cred,
        vl_tot_creditos,
        vl_aj_creditos,
        vl_tot_aj_creditos,
        vl_estornos_deb,
        vl_sld_credor_ant,
        vl_sld_apurado,
        vl_tot_ded,
        vl_icms_recolher,
        vl_sld_credor_transportar,
        deb_esp
    ]
);
register_model!(RegE110, "e110");
