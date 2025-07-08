use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d300::dsl as schema;
use crate::schemas::reg_d300::table;
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
#[diesel(table_name = crate::schemas::reg_d300::dsl)]
pub struct RegD300 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc_ini: Option<String>,
    pub num_doc_fin: Option<String>,
    pub cst_icms: Option<String>,
    pub cfop: Option<String>,
    pub aliq_icms: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_opr: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_serv: Option<String>,
    pub vl_seg: Option<String>,
    pub vl_out_desp: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_red_bc: Option<String>,
    pub cod_obs: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for RegD300 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD300 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            ser: get_field(&fields, 3),
            sub: get_field(&fields, 4),
            num_doc_ini: get_field(&fields, 5),
            num_doc_fin: get_field(&fields, 6),
            cst_icms: get_field(&fields, 7),
            cfop: get_field(&fields, 8),
            aliq_icms: get_field(&fields, 9),
            dt_doc: get_field(&fields, 10),
            vl_opr: get_field(&fields, 11),
            vl_desc: get_field(&fields, 12),
            vl_serv: get_field(&fields, 13),
            vl_seg: get_field(&fields, 14),
            vl_out_desp: get_field(&fields, 15),
            vl_bc_icms: get_field(&fields, 16),
            vl_icms: get_field(&fields, 17),
            vl_red_bc: get_field(&fields, 18),
            cod_obs: get_field(&fields, 19),
            cod_cta: get_field(&fields, 20),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD300>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD300::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD300::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc_ini.eq(&self.num_doc_ini),
                    schema::num_doc_fin.eq(&self.num_doc_fin),
                    schema::cst_icms.eq(&self.cst_icms),
                    schema::cfop.eq(&self.cfop),
                    schema::aliq_icms.eq(&self.aliq_icms),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_opr.eq(&self.vl_opr),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_serv.eq(&self.vl_serv),
                    schema::vl_seg.eq(&self.vl_seg),
                    schema::vl_out_desp.eq(&self.vl_out_desp),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_red_bc.eq(&self.vl_red_bc),
                    schema::cod_obs.eq(&self.cod_obs),
                    schema::cod_cta.eq(&self.cod_cta),
                ))
                .execute(&mut DB_POOL.lock().await.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.lock().await.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "RegD300".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD300 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD300, [reg, cod_mod, ser, sub, num_doc_ini, num_doc_fin, cst_icms, cfop, aliq_icms, dt_doc, vl_opr, vl_desc, vl_serv, vl_seg, vl_out_desp, vl_bc_icms, vl_icms, vl_red_bc, cod_obs, cod_cta]);
register_model!(RegD300, "d300");
