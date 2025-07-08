use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d150::dsl as schema;
use crate::schemas::reg_d150::table;
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
#[diesel(table_name = crate::schemas::reg_d150::dsl)]
pub struct RegD150 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mun_orig: Option<String>,
    pub cod_mun_dest: Option<String>,
    pub veic_id: Option<String>,
    pub viagem: Option<String>,
    pub ind_tfa: Option<String>,
    pub vl_peso_tx: Option<String>,
    pub vl_tx_terr: Option<String>,
    pub vl_tx_red: Option<String>,
    pub vl_out: Option<String>,
    pub vl_tx_adv: Option<String>,
}

#[async_trait]
impl Model for RegD150 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD150 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mun_orig: get_field(&fields, 2),
            cod_mun_dest: get_field(&fields, 3),
            veic_id: get_field(&fields, 4),
            viagem: get_field(&fields, 5),
            ind_tfa: get_field(&fields, 6),
            vl_peso_tx: get_field(&fields, 7),
            vl_tx_terr: get_field(&fields, 8),
            vl_tx_red: get_field(&fields, 9),
            vl_out: get_field(&fields, 10),
            vl_tx_adv: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD150>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD150::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD150::as_select())
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
                    schema::cod_mun_orig.eq(&self.cod_mun_orig),
                    schema::cod_mun_dest.eq(&self.cod_mun_dest),
                    schema::veic_id.eq(&self.veic_id),
                    schema::viagem.eq(&self.viagem),
                    schema::ind_tfa.eq(&self.ind_tfa),
                    schema::vl_peso_tx.eq(&self.vl_peso_tx),
                    schema::vl_tx_terr.eq(&self.vl_tx_terr),
                    schema::vl_tx_red.eq(&self.vl_tx_red),
                    schema::vl_out.eq(&self.vl_out),
                    schema::vl_tx_adv.eq(&self.vl_tx_adv),
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
        "RegD150".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD150 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD150, [reg, cod_mun_orig, cod_mun_dest, veic_id, viagem, ind_tfa, vl_peso_tx, vl_tx_terr, vl_tx_red, vl_out, vl_tx_adv]);
register_model!(RegD150, "d150");
