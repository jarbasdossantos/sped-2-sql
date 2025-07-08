use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1100::dsl as schema;
use crate::schemas::reg_1100::table;
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
#[diesel(table_name = crate::schemas::reg_1100::dsl)]
pub struct Reg1100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_doc: Option<String>,
    pub nro_de: Option<String>,
    pub dt_de: Option<String>,
    pub nat_exp: Option<String>,
    pub nro_re: Option<String>,
    pub dt_re: Option<String>,
    pub chc_emb: Option<String>,
    pub dt_chc: Option<String>,
    pub dt_avb: Option<String>,
    pub tp_chc: Option<String>,
    pub pais: Option<String>,
}

#[async_trait]
impl Model for Reg1100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_doc: get_field(&fields, 2),
            nro_de: get_field(&fields, 3),
            dt_de: get_field(&fields, 4),
            nat_exp: get_field(&fields, 5),
            nro_re: get_field(&fields, 6),
            dt_re: get_field(&fields, 7),
            chc_emb: get_field(&fields, 8),
            dt_chc: get_field(&fields, 9),
            dt_avb: get_field(&fields, 10),
            tp_chc: get_field(&fields, 11),
            pais: get_field(&fields, 12),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1100>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1100::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1100::as_select())
                .load(&mut conn)?)
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
                    schema::ind_doc.eq(&self.ind_doc),
                    schema::nro_de.eq(&self.nro_de),
                    schema::dt_de.eq(&self.dt_de),
                    schema::nat_exp.eq(&self.nat_exp),
                    schema::nro_re.eq(&self.nro_re),
                    schema::dt_re.eq(&self.dt_re),
                    schema::chc_emb.eq(&self.chc_emb),
                    schema::dt_chc.eq(&self.dt_chc),
                    schema::dt_avb.eq(&self.dt_avb),
                    schema::tp_chc.eq(&self.tp_chc),
                    schema::pais.eq(&self.pais),
                ))
                .execute(&mut conn)?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Reg1100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1100, [reg, ind_doc, nro_de, dt_de, nat_exp, nro_re, dt_re, chc_emb, dt_chc, dt_avb, tp_chc, pais]);
register_model!(Reg1100, "1100");
