use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c600::dsl as schema;
use crate::schemas::reg_c600::table;
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
#[diesel(table_name = crate::schemas::reg_c600::dsl)]
pub struct RegC600 {
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
    pub qtd_canc: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_desc: Option<String>,
    pub cons: Option<String>,
    pub vl_forn: Option<String>,
    pub vl_serv_nt: Option<String>,
    pub vl_terc: Option<String>,
    pub vl_da: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
}

#[async_trait]
impl Model for RegC600 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC600 {
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
            qtd_canc: get_field(&fields, 8),
            dt_doc: get_field(&fields, 9),
            vl_doc: get_field(&fields, 10),
            vl_desc: get_field(&fields, 11),
            cons: get_field(&fields, 12),
            vl_forn: get_field(&fields, 13),
            vl_serv_nt: get_field(&fields, 14),
            vl_terc: get_field(&fields, 15),
            vl_da: get_field(&fields, 16),
            vl_bc_icms: get_field(&fields, 17),
            vl_icms: get_field(&fields, 18),
            vl_bc_icms_st: get_field(&fields, 19),
            vl_icms_st: get_field(&fields, 20),
            vl_pis: get_field(&fields, 21),
            vl_cofins: get_field(&fields, 22),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC600>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC600::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC600::as_select())
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
                    schema::cod_mun.eq(&self.cod_mun),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::cod_cons.eq(&self.cod_cons),
                    schema::qtd_cons.eq(&self.qtd_cons),
                    schema::qtd_canc.eq(&self.qtd_canc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::cons.eq(&self.cons),
                    schema::vl_forn.eq(&self.vl_forn),
                    schema::vl_serv_nt.eq(&self.vl_serv_nt),
                    schema::vl_terc.eq(&self.vl_terc),
                    schema::vl_da.eq(&self.vl_da),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_bc_icms_st.eq(&self.vl_bc_icms_st),
                    schema::vl_icms_st.eq(&self.vl_icms_st),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
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
        "RegC600".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC600 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC600, [reg, cod_mod, cod_mun, ser, sub, cod_cons, qtd_cons, qtd_canc, dt_doc, vl_doc, vl_desc, cons, vl_forn, vl_serv_nt, vl_terc, vl_da, vl_bc_icms, vl_icms, vl_bc_icms_st, vl_icms_st, vl_pis, vl_cofins]);
register_model!(RegC600, "c600");
