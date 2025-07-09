use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1310::dsl as schema;
use crate::schemas::reg_1310::table;
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
#[diesel(table_name = crate::schemas::reg_1310::dsl)]
pub struct Reg1310 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub num_tanque: Option<String>,
    pub estq_abert: Option<String>,
    pub vol_entr: Option<String>,
    pub vol_disp: Option<String>,
    pub vol_saidas: Option<String>,
    pub estq_escr: Option<String>,
    pub val_aj_perda: Option<String>,
    pub val_aj_ganho: Option<String>,
    pub fech_fisico: Option<String>,
}

#[async_trait]
impl Model for Reg1310 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1310 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            num_tanque: get_field(&fields, 2),
            estq_abert: get_field(&fields, 3),
            vol_entr: get_field(&fields, 4),
            vol_disp: get_field(&fields, 5),
            vol_saidas: get_field(&fields, 6),
            estq_escr: get_field(&fields, 7),
            val_aj_perda: get_field(&fields, 8),
            val_aj_ganho: get_field(&fields, 9),
            fech_fisico: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1310>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1310::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1310::as_select())
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
                    schema::num_tanque.eq(&self.num_tanque),
                    schema::estq_abert.eq(&self.estq_abert),
                    schema::vol_entr.eq(&self.vol_entr),
                    schema::vol_disp.eq(&self.vol_disp),
                    schema::vol_saidas.eq(&self.vol_saidas),
                    schema::estq_escr.eq(&self.estq_escr),
                    schema::val_aj_perda.eq(&self.val_aj_perda),
                    schema::val_aj_ganho.eq(&self.val_aj_ganho),
                    schema::fech_fisico.eq(&self.fech_fisico),
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
        "Reg1310".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1310 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1310, [reg, num_tanque, estq_abert, vol_entr, vol_disp, vol_saidas, estq_escr, val_aj_perda, val_aj_ganho, fech_fisico]);
register_model!(Reg1310, "1310");
