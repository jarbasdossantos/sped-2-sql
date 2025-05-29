use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c700::reg_c700::dsl as schema;
use crate::schemas::reg_c700::reg_c700::table;
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
#[diesel(table_name = crate::schemas::reg_c700::reg_c700::dsl)]
pub struct RegC700 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub nro_ord_ini: Option<String>,
    pub nro_ord_fin: Option<String>,
    pub dt_doc_ini: Option<String>,
    pub dt_doc_fin: Option<String>,
    pub nom_mest: Option<String>,
    pub chv_cod_dig: Option<String>,
}

#[async_trait]
impl Model for RegC700 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC700 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            ser: get_field(&fields, 3),
            nro_ord_ini: get_field(&fields, 4),
            nro_ord_fin: get_field(&fields, 5),
            dt_doc_ini: get_field(&fields, 6),
            dt_doc_fin: get_field(&fields, 7),
            nom_mest: get_field(&fields, 8),
            chv_cod_dig: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC700>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC700::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC700::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::nro_ord_ini.eq(&self.nro_ord_ini),
                    schema::nro_ord_fin.eq(&self.nro_ord_fin),
                    schema::dt_doc_ini.eq(&self.dt_doc_ini),
                    schema::dt_doc_fin.eq(&self.dt_doc_fin),
                    schema::nom_mest.eq(&self.nom_mest),
                    schema::chv_cod_dig.eq(&self.chv_cod_dig),
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
        "RegC700".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC700 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegC700,
    [
        reg,
        cod_mod,
        ser,
        nro_ord_ini,
        nro_ord_fin,
        dt_doc_ini,
        dt_doc_fin,
        nom_mest,
        chv_cod_dig
    ]
);
register_model!(RegC700, "c700");
