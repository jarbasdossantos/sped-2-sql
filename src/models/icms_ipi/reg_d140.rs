use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d140::reg_d140::dsl as schema;
use crate::schemas::reg_d140::reg_d140::table;
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
#[diesel(table_name = crate::schemas::reg_d140::reg_d140::dsl)]
pub struct RegD140 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_part_consg: Option<String>,
    pub cod_mun_orig: Option<String>,
    pub cod_mun_dest: Option<String>,
    pub ind_veic: Option<String>,
    pub veic_id: Option<String>,
    pub ind_nav: Option<String>,
    pub viagem: Option<String>,
    pub vl_frt_liq: Option<String>,
    pub vl_desp_port: Option<String>,
    pub vl_desp_car_desc: Option<String>,
    pub vl_out: Option<String>,
    pub vl_frt_brt: Option<String>,
    pub vl_frt_mm: Option<String>,
}

#[async_trait]
impl Model for RegD140 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD140 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_part_consg: get_field(&fields, 2),
            cod_mun_orig: get_field(&fields, 3),
            cod_mun_dest: get_field(&fields, 4),
            ind_veic: get_field(&fields, 5),
            veic_id: get_field(&fields, 6),
            ind_nav: get_field(&fields, 7),
            viagem: get_field(&fields, 8),
            vl_frt_liq: get_field(&fields, 9),
            vl_desp_port: get_field(&fields, 10),
            vl_desp_car_desc: get_field(&fields, 11),
            vl_out: get_field(&fields, 12),
            vl_frt_brt: get_field(&fields, 13),
            vl_frt_mm: get_field(&fields, 14),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD140>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD140::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD140::as_select())
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
                    schema::cod_part_consg.eq(&self.cod_part_consg),
                    schema::cod_mun_orig.eq(&self.cod_mun_orig),
                    schema::cod_mun_dest.eq(&self.cod_mun_dest),
                    schema::ind_veic.eq(&self.ind_veic),
                    schema::veic_id.eq(&self.veic_id),
                    schema::ind_nav.eq(&self.ind_nav),
                    schema::viagem.eq(&self.viagem),
                    schema::vl_frt_liq.eq(&self.vl_frt_liq),
                    schema::vl_desp_port.eq(&self.vl_desp_port),
                    schema::vl_desp_car_desc.eq(&self.vl_desp_car_desc),
                    schema::vl_out.eq(&self.vl_out),
                    schema::vl_frt_brt.eq(&self.vl_frt_brt),
                    schema::vl_frt_mm.eq(&self.vl_frt_mm),
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
        "RegD140".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD140 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegD140,
    [
        reg,
        cod_part_consg,
        cod_mun_orig,
        cod_mun_dest,
        ind_veic,
        veic_id,
        ind_nav,
        viagem,
        vl_frt_liq,
        vl_desp_port,
        vl_desp_car_desc,
        vl_out,
        vl_frt_brt,
        vl_frt_mm
    ]
);
register_model!(RegD140, "d140");
