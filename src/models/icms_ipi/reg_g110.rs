use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_g110::reg_g110::dsl as schema;
use crate::schemas::reg_g110::reg_g110::table;
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
#[diesel(table_name = crate::schemas::reg_g110::reg_g110::dsl)]
pub struct RegG110 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fin: Option<String>,
    pub saldo_in_icms: Option<String>,
    pub som_parc: Option<String>,
    pub vl_trib_exp: Option<String>,
    pub vl_total: Option<String>,
    pub ind_per_sai: Option<String>,
    pub icms_aprop: Option<String>,
    pub som_icms_oc: Option<String>,
}

#[async_trait]
impl Model for RegG110 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegG110 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    dt_ini: get_field(&fields, 2),
        dt_fin: get_field(&fields, 3),
        saldo_in_icms: get_field(&fields, 4),
        som_parc: get_field(&fields, 5),
        vl_trib_exp: get_field(&fields, 6),
        vl_total: get_field(&fields, 7),
        ind_per_sai: get_field(&fields, 8),
        icms_aprop: get_field(&fields, 9),
        som_icms_oc: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegG110>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegG110::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegG110::as_select())
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
schema::dt_fin.eq(&self.dt_fin),
schema::saldo_in_icms.eq(&self.saldo_in_icms),
schema::som_parc.eq(&self.som_parc),
schema::vl_trib_exp.eq(&self.vl_trib_exp),
schema::vl_total.eq(&self.vl_total),
schema::ind_per_sai.eq(&self.ind_per_sai),
schema::icms_aprop.eq(&self.icms_aprop),
schema::som_icms_oc.eq(&self.som_icms_oc),
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
        "RegG110".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegG110 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegG110, [reg, dt_ini, dt_fin, saldo_in_icms, som_parc, vl_trib_exp, vl_total, ind_per_sai, icms_aprop, som_icms_oc]);
register_model!(RegG110, "g110");
