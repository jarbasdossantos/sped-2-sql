use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d130::dsl as schema;
use crate::schemas::reg_d130::table;
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
#[diesel(table_name = crate::schemas::reg_d130::dsl)]
pub struct RegD130 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_part_consg: Option<String>,
    pub cod_part_red: Option<String>,
    pub ind_frt_red: Option<String>,
    pub cod_mun_orig: Option<String>,
    pub cod_mun_dest: Option<String>,
    pub veic_id: Option<String>,
    pub vl_liq_frt: Option<String>,
    pub vl_sec_cat: Option<String>,
    pub vl_desp: Option<String>,
    pub vl_pedg: Option<String>,
    pub vl_out: Option<String>,
    pub vl_frt: Option<String>,
    pub uf_id: Option<String>,
}

#[async_trait]
impl Model for RegD130 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD130 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_part_consg: get_field(&fields, 2),
            cod_part_red: get_field(&fields, 3),
            ind_frt_red: get_field(&fields, 4),
            cod_mun_orig: get_field(&fields, 5),
            cod_mun_dest: get_field(&fields, 6),
            veic_id: get_field(&fields, 7),
            vl_liq_frt: get_field(&fields, 8),
            vl_sec_cat: get_field(&fields, 9),
            vl_desp: get_field(&fields, 10),
            vl_pedg: get_field(&fields, 11),
            vl_out: get_field(&fields, 12),
            vl_frt: get_field(&fields, 13),
            uf_id: get_field(&fields, 14),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD130>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD130::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD130::as_select())
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
                    schema::cod_part_consg.eq(&self.cod_part_consg),
                    schema::cod_part_red.eq(&self.cod_part_red),
                    schema::ind_frt_red.eq(&self.ind_frt_red),
                    schema::cod_mun_orig.eq(&self.cod_mun_orig),
                    schema::cod_mun_dest.eq(&self.cod_mun_dest),
                    schema::veic_id.eq(&self.veic_id),
                    schema::vl_liq_frt.eq(&self.vl_liq_frt),
                    schema::vl_sec_cat.eq(&self.vl_sec_cat),
                    schema::vl_desp.eq(&self.vl_desp),
                    schema::vl_pedg.eq(&self.vl_pedg),
                    schema::vl_out.eq(&self.vl_out),
                    schema::vl_frt.eq(&self.vl_frt),
                    schema::uf_id.eq(&self.uf_id),
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
        "RegD130".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD130 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD130, [reg, cod_part_consg, cod_part_red, ind_frt_red, cod_mun_orig, cod_mun_dest, veic_id, vl_liq_frt, vl_sec_cat, vl_desp, vl_pedg, vl_out, vl_frt, uf_id]);
register_model!(RegD130, "d130");
