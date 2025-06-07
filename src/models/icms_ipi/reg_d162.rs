use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d162::dsl as schema;
use crate::schemas::reg_d162::table;
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
#[diesel(table_name = crate::schemas::reg_d162::dsl)]
pub struct RegD162 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_merc: Option<String>,
    pub qtd_vol: Option<String>,
    pub peso_brt: Option<String>,
    pub peso_liq: Option<String>,
}

#[async_trait]
impl Model for RegD162 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD162 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            ser: get_field(&fields, 3),
            num_doc: get_field(&fields, 4),
            dt_doc: get_field(&fields, 5),
            vl_doc: get_field(&fields, 6),
            vl_merc: get_field(&fields, 7),
            qtd_vol: get_field(&fields, 8),
            peso_brt: get_field(&fields, 9),
            peso_liq: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD162>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD162::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD162::as_select())
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
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_merc.eq(&self.vl_merc),
                    schema::qtd_vol.eq(&self.qtd_vol),
                    schema::peso_brt.eq(&self.peso_brt),
                    schema::peso_liq.eq(&self.peso_liq),
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
        "RegD162".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD162 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD162, [reg, cod_mod, ser, num_doc, dt_doc, vl_doc, vl_merc, qtd_vol, peso_brt, peso_liq]);
register_model!(RegD162, "d162");
