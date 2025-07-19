use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1391::dsl as schema;
use crate::schemas::reg_1391::table;
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
#[diesel(table_name = crate::schemas::reg_1391::dsl)]
pub struct Reg1391 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_registro: Option<String>,
    pub qtd_moid: Option<String>,
    pub estq_ini: Option<String>,
    pub qtd_produz: Option<String>,
    pub ent_anid_hid: Option<String>,
    pub outr_entr: Option<String>,
    pub perda: Option<String>,
    pub cons: Option<String>,
    pub sai_ani_hid: Option<String>,
    pub saidas: Option<String>,
    pub estq_fin: Option<String>,
    pub estq_ini_mel: Option<String>,
    pub prod_dia_mel: Option<String>,
    pub util_mel: Option<String>,
    pub prod_alc_mel: Option<String>,
    pub obs: Option<String>,
}

#[async_trait]
impl Model for Reg1391 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1391 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            dt_registro: get_field(&fields, 2),
            qtd_moid: get_field(&fields, 3),
            estq_ini: get_field(&fields, 4),
            qtd_produz: get_field(&fields, 5),
            ent_anid_hid: get_field(&fields, 6),
            outr_entr: get_field(&fields, 7),
            perda: get_field(&fields, 8),
            cons: get_field(&fields, 9),
            sai_ani_hid: get_field(&fields, 10),
            saidas: get_field(&fields, 11),
            estq_fin: get_field(&fields, 12),
            estq_ini_mel: get_field(&fields, 13),
            prod_dia_mel: get_field(&fields, 14),
            util_mel: get_field(&fields, 15),
            prod_alc_mel: get_field(&fields, 16),
            obs: get_field(&fields, 17),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<Reg1391>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1391::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1391::as_select())
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
                    schema::dt_registro.eq(&self.dt_registro),
                    schema::qtd_moid.eq(&self.qtd_moid),
                    schema::estq_ini.eq(&self.estq_ini),
                    schema::qtd_produz.eq(&self.qtd_produz),
                    schema::ent_anid_hid.eq(&self.ent_anid_hid),
                    schema::outr_entr.eq(&self.outr_entr),
                    schema::perda.eq(&self.perda),
                    schema::cons.eq(&self.cons),
                    schema::sai_ani_hid.eq(&self.sai_ani_hid),
                    schema::saidas.eq(&self.saidas),
                    schema::estq_fin.eq(&self.estq_fin),
                    schema::estq_ini_mel.eq(&self.estq_ini_mel),
                    schema::prod_dia_mel.eq(&self.prod_dia_mel),
                    schema::util_mel.eq(&self.util_mel),
                    schema::prod_alc_mel.eq(&self.prod_alc_mel),
                    schema::obs.eq(&self.obs),
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
        "Reg1391".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1391 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1391, [reg, dt_registro, qtd_moid, estq_ini, qtd_produz, ent_anid_hid, outr_entr, perda, cons, sai_ani_hid, saidas, estq_fin, estq_ini_mel, prod_dia_mel, util_mel, prod_alc_mel, obs]);
register_model!(Reg1391, "1391");
