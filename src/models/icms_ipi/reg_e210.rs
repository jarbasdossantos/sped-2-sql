use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_e210::dsl as schema;
use crate::schemas::reg_e210::table;
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
#[diesel(table_name = crate::schemas::reg_e210::dsl)]
pub struct RegE210 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_mov_st: Option<String>,
    pub vl_sld_cred_ant_st: Option<String>,
    pub vl_devol_st: Option<String>,
    pub vl_ressarc_st: Option<String>,
    pub vl_out_cred_st: Option<String>,
    pub vl_aj_creditos_st: Option<String>,
    pub vl_retencao_st: Option<String>,
    pub vl_out_deb_st: Option<String>,
    pub vl_aj_debitos_st: Option<String>,
    pub vl_sld_dev_ant_st: Option<String>,
    pub vl_deducoes_st: Option<String>,
    pub vl_icms_recol_st: Option<String>,
    pub vl_sld_cred_st_transportar: Option<String>,
    pub deb_esp_st: Option<String>,
}

#[async_trait]
impl Model for RegE210 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegE210 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_mov_st: get_field(&fields, 2),
            vl_sld_cred_ant_st: get_field(&fields, 3),
            vl_devol_st: get_field(&fields, 4),
            vl_ressarc_st: get_field(&fields, 5),
            vl_out_cred_st: get_field(&fields, 6),
            vl_aj_creditos_st: get_field(&fields, 7),
            vl_retencao_st: get_field(&fields, 8),
            vl_out_deb_st: get_field(&fields, 9),
            vl_aj_debitos_st: get_field(&fields, 10),
            vl_sld_dev_ant_st: get_field(&fields, 11),
            vl_deducoes_st: get_field(&fields, 12),
            vl_icms_recol_st: get_field(&fields, 13),
            vl_sld_cred_st_transportar: get_field(&fields, 14),
            deb_esp_st: get_field(&fields, 15),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegE210>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegE210::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegE210::as_select())
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
                    schema::ind_mov_st.eq(&self.ind_mov_st),
                    schema::vl_sld_cred_ant_st.eq(&self.vl_sld_cred_ant_st),
                    schema::vl_devol_st.eq(&self.vl_devol_st),
                    schema::vl_ressarc_st.eq(&self.vl_ressarc_st),
                    schema::vl_out_cred_st.eq(&self.vl_out_cred_st),
                    schema::vl_aj_creditos_st.eq(&self.vl_aj_creditos_st),
                    schema::vl_retencao_st.eq(&self.vl_retencao_st),
                    schema::vl_out_deb_st.eq(&self.vl_out_deb_st),
                    schema::vl_aj_debitos_st.eq(&self.vl_aj_debitos_st),
                    schema::vl_sld_dev_ant_st.eq(&self.vl_sld_dev_ant_st),
                    schema::vl_deducoes_st.eq(&self.vl_deducoes_st),
                    schema::vl_icms_recol_st.eq(&self.vl_icms_recol_st),
                    schema::vl_sld_cred_st_transportar.eq(&self.vl_sld_cred_st_transportar),
                    schema::deb_esp_st.eq(&self.deb_esp_st),
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
        "RegE210".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegE210 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegE210, [reg, ind_mov_st, vl_sld_cred_ant_st, vl_devol_st, vl_ressarc_st, vl_out_cred_st, vl_aj_creditos_st, vl_retencao_st, vl_out_deb_st, vl_aj_debitos_st, vl_sld_dev_ant_st, vl_deducoes_st, vl_icms_recol_st, vl_sld_cred_st_transportar, deb_esp_st]);
register_model!(RegE210, "e210");
