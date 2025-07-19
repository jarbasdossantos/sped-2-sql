use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d600::dsl as schema;
use crate::schemas::reg_d600::table;
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
#[diesel(table_name = crate::schemas::reg_d600::dsl)]
pub struct RegD600 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_mun: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub cod_cons: Option<String>,
    pub qtd_cons: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_serv: Option<String>,
    pub vl_serv_nt: Option<String>,
    pub vl_terc: Option<String>,
    pub vl_da: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
}

#[async_trait]
impl Model for RegD600 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD600 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            cod_mun: get_field(&fields, 3),
            ser: get_field(&fields, 4),
            sub: get_field(&fields, 5),
            cod_cons: get_field(&fields, 6),
            qtd_cons: get_field(&fields, 7),
            dt_doc: get_field(&fields, 8),
            vl_doc: get_field(&fields, 9),
            vl_desc: get_field(&fields, 10),
            vl_serv: get_field(&fields, 11),
            vl_serv_nt: get_field(&fields, 12),
            vl_terc: get_field(&fields, 13),
            vl_da: get_field(&fields, 14),
            vl_bc_icms: get_field(&fields, 15),
            vl_icms: get_field(&fields, 16),
            vl_pis: get_field(&fields, 17),
            vl_cofins: get_field(&fields, 18),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegD600>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD600::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD600::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::cod_mun.eq(&self.cod_mun),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::cod_cons.eq(&self.cod_cons),
                    schema::qtd_cons.eq(&self.qtd_cons),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_serv.eq(&self.vl_serv),
                    schema::vl_serv_nt.eq(&self.vl_serv_nt),
                    schema::vl_terc.eq(&self.vl_terc),
                    schema::vl_da.eq(&self.vl_da),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
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
        "RegD600".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD600 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD600, [reg, cod_mod, cod_mun, ser, sub, cod_cons, qtd_cons, dt_doc, vl_doc, vl_desc, vl_serv, vl_serv_nt, vl_terc, vl_da, vl_bc_icms, vl_icms, vl_pis, vl_cofins]);
register_model!(RegD600, "d600");
