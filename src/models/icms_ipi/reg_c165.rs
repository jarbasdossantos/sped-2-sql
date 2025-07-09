use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c165::dsl as schema;
use crate::schemas::reg_c165::table;
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
#[diesel(table_name = crate::schemas::reg_c165::dsl)]
pub struct RegC165 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_part: Option<String>,
    pub veic_id: Option<String>,
    pub cod_aut: Option<String>,
    pub nr_passe: Option<String>,
    pub hora: Option<String>,
    pub temper: Option<String>,
    pub qtd_vol: Option<String>,
    pub peso_brt: Option<String>,
    pub peso_liq: Option<String>,
    pub nom_mot: Option<String>,
    pub cpf: Option<String>,
    pub uf_id: Option<String>,
}

#[async_trait]
impl Model for RegC165 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC165 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_part: get_field(&fields, 2),
            veic_id: get_field(&fields, 3),
            cod_aut: get_field(&fields, 4),
            nr_passe: get_field(&fields, 5),
            hora: get_field(&fields, 6),
            temper: get_field(&fields, 7),
            qtd_vol: get_field(&fields, 8),
            peso_brt: get_field(&fields, 9),
            peso_liq: get_field(&fields, 10),
            nom_mot: get_field(&fields, 11),
            cpf: get_field(&fields, 12),
            uf_id: get_field(&fields, 13),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC165>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC165::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC165::as_select())
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
                    schema::cod_part.eq(&self.cod_part),
                    schema::veic_id.eq(&self.veic_id),
                    schema::cod_aut.eq(&self.cod_aut),
                    schema::nr_passe.eq(&self.nr_passe),
                    schema::hora.eq(&self.hora),
                    schema::temper.eq(&self.temper),
                    schema::qtd_vol.eq(&self.qtd_vol),
                    schema::peso_brt.eq(&self.peso_brt),
                    schema::peso_liq.eq(&self.peso_liq),
                    schema::nom_mot.eq(&self.nom_mot),
                    schema::cpf.eq(&self.cpf),
                    schema::uf_id.eq(&self.uf_id),
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
        "RegC165".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC165 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC165, [reg, cod_part, veic_id, cod_aut, nr_passe, hora, temper, qtd_vol, peso_brt, peso_liq, nom_mot, cpf, uf_id]);
register_model!(RegC165, "c165");
